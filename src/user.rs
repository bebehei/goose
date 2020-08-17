use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use std::sync::atomic::Ordering;
use std::time;
use tokio::sync::mpsc;

use crate::get_worker_id;
use crate::goose::{GooseTaskFunction, GooseTaskSet, GooseUser, GooseUserCommand};
use crate::metrics::{GooseMetric, GooseRawTask};

pub async fn user_main(
    thread_number: usize,
    thread_task_set: GooseTaskSet,
    mut thread_user: GooseUser,
    mut thread_receiver: mpsc::UnboundedReceiver<GooseUserCommand>,
    worker: bool,
) {
    if worker {
        info!(
            "[{}] launching user {} from {}...",
            get_worker_id(),
            thread_number,
            thread_task_set.name
        );
    } else {
        info!(
            "launching user {} from {}...",
            thread_number, thread_task_set.name
        );
    }

    // User is starting, first invoke the weighted on_start tasks.
    if !thread_user.weighted_on_start_tasks.is_empty() {
        for mut sequence in thread_user.weighted_on_start_tasks.clone() {
            if sequence.len() > 1 {
                sequence.shuffle(&mut thread_rng());
            }
            for task_index in &sequence {
                // Determine which task we're going to run next.
                let thread_task_name = &thread_task_set.tasks[*task_index].name;
                let function = &thread_task_set.tasks[*task_index].function;
                debug!(
                    "launching on_start {} task from {}",
                    thread_task_name, thread_task_set.name
                );
                if thread_task_name != "" {
                    thread_user.task_request_name = Some(thread_task_name.to_string());
                }
                // Invoke the task function.
                invoke_task_function(function, &thread_user, *task_index, thread_task_name).await;
            }
        }
    }

    // Repeatedly loop through all available tasks in a random order.
    let mut thread_continue: bool = true;
    let mut weighted_bucket = thread_user.weighted_bucket.load(Ordering::SeqCst);
    let mut weighted_bucket_position = thread_user.weighted_bucket_position.load(Ordering::SeqCst);
    if thread_user.weighted_tasks.is_empty() {
        // Handle the edge case where a load test doesn't define any normal tasks.
        thread_continue = false;
    }
    while thread_continue {
        // Weighted_tasks is divided into buckets of tasks sorted by sequence, and then all non-sequenced tasks.
        if thread_user.weighted_tasks[weighted_bucket].len() <= weighted_bucket_position {
            // This bucket is exhausted, move on to position 0 of the next bucket.
            weighted_bucket_position = 0;
            thread_user
                .weighted_bucket_position
                .store(weighted_bucket_position, Ordering::SeqCst);

            weighted_bucket += 1;
            if thread_user.weighted_tasks.len() <= weighted_bucket {
                weighted_bucket = 0;
            }
            thread_user
                .weighted_bucket
                .store(weighted_bucket_position, Ordering::SeqCst);
            // Shuffle new bucket before we walk through the tasks.
            thread_user.weighted_tasks[weighted_bucket].shuffle(&mut thread_rng());
            debug!(
                "re-shuffled {} tasks: {:?}",
                &thread_task_set.name, thread_user.weighted_tasks[weighted_bucket]
            );
        }

        // Determine which task we're going to run next.
        let thread_weighted_task =
            thread_user.weighted_tasks[weighted_bucket][weighted_bucket_position];
        let thread_task_name = &thread_task_set.tasks[thread_weighted_task].name;
        let function = &thread_task_set.tasks[thread_weighted_task].function;
        debug!(
            "launching {} task from {}",
            thread_task_name, thread_task_set.name
        );
        // If task name is set, it will be used for storing request statistics instead of the raw url.
        if thread_task_name != "" {
            thread_user.task_request_name = Some(thread_task_name.to_string());
        }

        // Invoke the task function.
        invoke_task_function(
            function,
            &thread_user,
            thread_weighted_task,
            thread_task_name,
        )
        .await;

        // Prepare to sleep for a random value from min_wait to max_wait.
        let wait_time = if thread_user.max_wait > 0 {
            rand::thread_rng().gen_range(thread_user.min_wait, thread_user.max_wait)
        } else {
            0
        };
        // Counter to track how long we've slept, waking regularly to check for messages.
        let mut slept: usize = 0;

        // Check if the parent thread has sent us any messages.
        let mut in_sleep_loop = true;
        while in_sleep_loop {
            let mut message = thread_receiver.try_recv();
            while message.is_ok() {
                match message.unwrap() {
                    // Time to exit.
                    GooseUserCommand::EXIT => {
                        // No need to reset per-thread counters, we're exiting and memory will be freed
                        thread_continue = false;
                    }
                    command => {
                        debug!("ignoring unexpected GooseUserCommand: {:?}", command);
                    }
                }
                message = thread_receiver.try_recv();
            }
            if thread_continue && thread_user.max_wait > 0 {
                let sleep_duration = time::Duration::from_secs(1);
                debug!(
                    "user {} from {} sleeping {:?} second...",
                    thread_number, thread_task_set.name, sleep_duration
                );
                tokio::time::delay_for(sleep_duration).await;
                slept += 1;
                if slept > wait_time {
                    in_sleep_loop = false;
                }
            } else {
                in_sleep_loop = false;
            }
        }

        // Move to the next task in thread_user.weighted_tasks.
        weighted_bucket_position += 1;
        thread_user
            .weighted_bucket_position
            .store(weighted_bucket_position, Ordering::SeqCst);
    }

    // User is exiting, first invoke the weighted on_stop tasks.
    if !thread_user.weighted_on_stop_tasks.is_empty() {
        for mut sequence in thread_user.weighted_on_stop_tasks.clone() {
            if sequence.len() > 1 {
                sequence.shuffle(&mut thread_rng());
            }
            for task_index in &sequence {
                // Determine which task we're going to run next.
                let thread_task_name = &thread_task_set.tasks[*task_index].name;
                let function = &thread_task_set.tasks[*task_index].function;
                debug!(
                    "launching on_stop {} task from {}",
                    thread_task_name, thread_task_set.name
                );
                if thread_task_name != "" {
                    thread_user.task_request_name = Some(thread_task_name.to_string());
                }
                // Invoke the task function.
                invoke_task_function(function, &thread_user, *task_index, thread_task_name).await;
            }
        }
    }

    // Optional debug output when exiting.
    if worker {
        info!(
            "[{}] exiting user {} from {}...",
            get_worker_id(),
            thread_number,
            thread_task_set.name
        );
    } else {
        info!(
            "exiting user {} from {}...",
            thread_number, thread_task_set.name
        );
    }
}

// Invoke the task function, collecting task statistics.
async fn invoke_task_function(
    function: &GooseTaskFunction,
    thread_user: &GooseUser,
    task_index: usize,
    thread_task_name: &str,
) {
    let started = time::Instant::now();
    let mut raw_task = GooseRawTask::new(
        thread_user.started.elapsed().as_millis(),
        thread_user.task_sets_index,
        task_index,
        thread_task_name.to_string(),
        thread_user.weighted_users_index,
    );
    let success = function(&thread_user).await.is_ok();
    raw_task.set_time(started.elapsed().as_millis(), success);

    // Exit if all statistics or task statistics are disabled.
    if thread_user.config.no_metrics || thread_user.config.no_task_metrics {
        return;
    }

    // Otherwise send statistics to parent.
    if let Some(parent) = thread_user.channel_to_parent.clone() {
        // Best effort statistics.
        let _ = parent.send(GooseMetric::Task(raw_task));
    }
}
