//! Helpers and objects for building Goose load tests.
//! 
//! Goose manages load tests with a series of objects:
//! 
//! - [`GooseTest`](./struct.GooseTest.html) a global object that holds all task sets and client states.
//! - [`GooseTaskSet`](./struct.GooseTaskSet.html) each client is assigned a task set, which is a collection of tasks.
//! - [`GooseTask`](./struct.GooseTask.html) tasks define one or more web requests and are assigned to task sets.
//! - [`GooseClient`](./struct.GooseClient.html) a client state responsible for repeatedly running all tasks in the assigned task set.
//! - [`GooseRequest`](./struct.GooseRequest.html) optional statistics collected for each URL/method pair.
//! 
//! ## Creating Task Sets
//! 
//! A [`GooseTaskSet`](./struct.GooseTaskSet.html) is created by passing in a `&str` to the `new` function, for example:
//! 
//! ```rust
//!     let mut loadtest_tasks = GooseTaskSet::new("LoadtestTasks");
//! ```
//! 
//! ### Task Set Weight
//! 
//! A weight can be assigned to a task set, controlling how often it is assigned to client
//! threads. The larger the value of weight, the more it will be assigned to clients. In the
//! following example, `FooTasks` will be assigned to clients twice as often as `Bar` tasks.
//! We could have just added a weight of `2` to `FooTasks` and left the default weight of `1`
//! assigned to `BarTasks` for the same weighting:
//! 
//! ```rust
//!     let mut foo_tasks = GooseTaskSet::new("FooTasks").set_weight(10);
//!     let mut bar_tasks = GooseTaskSet::new("BarTasks").set_weight(5);
//! ```
//! 
//! ### Task Set Host
//! 
//! A default host can be assigned to a task set, which will be used only if the `--host`
//! CLI option is not set at run-time. For example, this can configure your load test to
//! run against your local development environment by default, allowing the `--host` option
//! to override host when you want to load test production. You can also assign different
//! hosts to different task sets if this is desirable:
//! 
//! ```rust
//!     foo_tasks.set_host("http://www.local");
//!     bar_tasks.set_host("http://www2.local");
//! ```
//! 
//! ### Task Set Wait Time
//! 
//! Wait time is specified as a low-high integer range. Each time a task completes in
//! the task set, the client will pause for a random number of seconds inclusively between
//! the low and high wait times. In the following example, Clients loading `foo` tasks will
//! sleep 0 to 3 seconds after each task completes, and Clients loading `bar` tasks will
//! sleep 5 to 10 seconds after each task completes.
//! 
//! ```rust
//!     foo_tasks.set_wait_time(0, 3);
//!     bar_tasks.set_host(5, 10);
//! ```
//! ## Creating Tasks
//! 
//! A [`GooseTask`](./struct.GooseTask.html) can be created with or without a name.
//! The name is used when displaying statistics about the load test. For example:
//! 
//! ```rust
//!     let mut a_task = GooseTask::new();
//!     let mut b_task = GooseTask::named("b");
//! ```
//! 
//! ### Task Name
//! 
//! A name can also be assigned (or changed) after a task is created, for example:
//! 
//! ```rust
//!     a_task.set_name("a");
//! ```
//! 
//! ### Task Weight
//! 
//! Individual tasks can be assigned a weight, controlling how often the task runs. The
//! larger the value of weight, the more it will run. In the following example, `a_task`
//! runs 3 times as often as `b_task`:
//! 
//! ```rust
//!     a_task.set_weight(9);
//!     b_task.set_weight(3);
//! ```
//! 
//! ### Task Sequence
//! 
//! Tasks can also be configured to run in a sequence. For example, a task with a sequence
//! value of `1` will always run before a task with a sequence value of `2`. Weight can
//! be applied to sequenced tasks, so for example a task with a weight of `2` and a sequence
//! of `1` will run two times before a task with a sequence of `2`. Task sets can contain
//! tasks with sequence values and without sequence values, and in this case all tasks with
//! a sequence value will run before tasks without a sequence value. In the folllowing example,
//! `a_task` runs before `b_task`, which runs before `c_task`:
//! 
//! ```rust
//!     a_task.set_sequence(1);
//!     b_task.set_sequence(2);
//!     let mut c_task = GooseTask::named("c");
//! ```
//! 
//! ### Task Function
//! 
//! All tasks must be associated with a function. Goose will invoke this function each time
//! the task is run.
//! 
//! ```rust
//!     a_task.set_function(a_task_function);
//!     b_task.set_function(b_task_function);
//!     // Re-use the same task function.
//!     c_task.set_function(b_task_function);
//! ```
//! 
//! The same task function can be assigned to multiple tasks and/or multiple task sets, if desired.
//! 
//! ### Task On Start
//! 
//! Tasks can be flagged to only run when a client first starts. This can be useful if you'd
//! like your load test to use a logged-in user. It is possible to assign sequences and weights
//! to `on_start` functions if you want to have multiple tasks run at start time, and/or the
//! tasks to run multiple times.
//! 
//! ```rust
//!     a_task.set_on_start();
//! ```
//! 
//! ### Task On Stop
//! 
//! Tasks can be flagged to only run when a client stops. This can be useful if you'd like your
//! load test to simluate a user logging out when it finishes. It is possible to assign sequences
//! and weights to `on_stop` functions if you want to have multiple tasks run at stop time, and/or
//! the tasks to run multiple times.
//! 
//! ```rust
//!     a_task.set_on_stop();
//! ```
//! 
//! ## Controlling Clients
//! 
//! When Goose starts, it creates on or more [`GooseClient`](./struct.GooseClient.html),
//! assigning a single [`GooseTaskSet`](./struct.GooseTaskSet.html) to each. This client is
//! then used to generate load. Behind the scenes, Goose is leveraging the Reqwest Blocking
//! client to load web pages, and Goose can therefor do anything Reqwest can do.
//! 
//! The most common request types are GET and POST, but HEAD, PUT, PATCH, and DELETE are also
//! fully supported.
//! 
//! ### GET
//! 
//! A HTTP GET request.
//! 
//! ```
//!     client.get("/path/to/foo");
//! ```
//! 
//! ### POST
//! 
//! A HTTP POST request.
//! 
//! ```
//!     client.post("/path/to/bar");
//! ```
//! 
//! ### HEAD
//! 
//! ### PUT
//! 
//! ### PATCH
//! 
//! ### DELETE

use std::collections::HashMap;
use std::time::Instant;

use http::StatusCode;
use http::method::Method;
use reqwest::blocking::{Client, Response, RequestBuilder};
use reqwest::Error;
use url::Url;

use crate::GooseConfiguration;

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

/// A global list of all Goose task sets in the load test.
#[derive(Clone)]
pub struct GooseTest {
    /// A vector containing one copy of each GooseTaskSet that will run during this load test.
    pub task_sets: Vec<GooseTaskSet>,
    /// A weighted vector containing a GooseClient object for each client that will run during this load test.
    pub weighted_clients: Vec<GooseClient>,
    /// A weighted vector of integers used to randomize the order that the GooseClient threads are launched.
    pub weighted_clients_order: Vec<usize>,
    /// An optional default host to run this load test against.
    pub host: Option<String>,
}
impl GooseTest {
    /// Sets up a Goose load test. This function should only be invoked one time. The returned state must
    /// be stored in a mutable value.
    /// 
    /// # Example
    /// ```rust
    ///     let mut goose_test = GooseTest::new();
    /// ```
    pub fn new() -> Self {
        let goose_tasksets = GooseTest { 
            task_sets: Vec::new(),
            weighted_clients: Vec::new(),
            weighted_clients_order: Vec::new(),
            host: None,
        };
        goose_tasksets
    }

    /// A GooseTest contains one or more GooseTaskSet. Each must be registered with this method,
    /// which will add a new GooseTaskSet to the task_sets vector.
    /// 
    /// # Example
    /// ```rust
    ///     let mut goose_test = GooseTest::new();
    ///     let mut example_tasks = GooseTaskSet::new("ExampleTasks");
    ///     example_tasks.register_task(GooseTask::new().set_function(example_task));
    ///     goose_test.register_taskset(example_tasks);
    /// 
    /// ```
    pub fn register_taskset(&mut self, mut taskset: GooseTaskSet) {
        taskset.task_sets_index = self.task_sets.len();
        self.task_sets.push(taskset);
    }

    /// Set a default host for the load test. If no `--host` flag is set when running the load test, this
    /// host will be pre-pended on all requests. For example, this can configure your load test to run
    /// against your local development environment by default, and the `--host` option could be used to
    /// override host when running the load test against production.
    /// 
    /// A default host can also be configured per task set.
    /// 
    /// # Example
    /// ```rust
    ///     let mut goose_test = GooseTest::new().set_host("http://10.1.1.42");
    /// ```
    pub fn set_host(mut self, host: &str) -> Self {
        trace!("set_host: {}", host);
        // Host validation happens in main() at startup.
        self.host = Some(host.to_string());
        self
    }
}

/// An individual task set.
#[derive(Clone)]
pub struct GooseTaskSet {
    /// The name of the task set.
    pub name: String,
    /// An integer reflecting where this task set lives in the GooseTest.task_sets vector.
    pub task_sets_index: usize,
    /// An integer value that controls the frequency that this task set will be assigned to a client.
    pub weight: usize,
    /// An integer value indicating the minimum number of seconds a client will sleep after running a task.
    pub min_wait: usize,
    /// An integer value indicating the maximum number of seconds a client will sleep after running a task.
    pub max_wait: usize,
    /// A vector containing one copy of each GooseTask that will run by clients running this task set.
    pub tasks: Vec<GooseTask>,
    /// A vector of vectors of integers, controlling the sequence and order GooseTasks are run.
    pub weighted_tasks: Vec<Vec<usize>>,
    /// A vector of vectors of integers, controlling the sequence and order on_start GooseTasks are run when a client starts.
    pub weighted_on_start_tasks: Vec<Vec<usize>>,
    /// A vector of vectors of integers, controlling the sequence and order on_stop GooseTasks are run when a client stops.
    pub weighted_on_stop_tasks: Vec<Vec<usize>>,
    /// An optional default host to run this TaskSet against.
    pub host: Option<String>,
}
impl GooseTaskSet {
    /// Creates a new GooseTaskSet. Once created, GooseTasks must be assigned to it, and finally it must be
    /// registered with the GooseTest object. The returned object must be stored in a mutable value.
    /// 
    /// # Example
    /// ```rust
    ///     let mut example_tasks = GooseTaskSet::new("ExampleTasks");
    /// ```
    pub fn new(name: &str) -> Self {
        trace!("new taskset: name: {}", &name);
        let task_set = GooseTaskSet { 
            name: name.to_string(),
            task_sets_index: usize::max_value(),
            weight: 1,
            min_wait: 0,
            max_wait: 0,
            tasks: Vec::new(),
            weighted_tasks: Vec::new(),
            weighted_on_start_tasks: Vec::new(),
            weighted_on_stop_tasks: Vec::new(),
            host: None,
        };
        task_set
    }

    /// Registers a GooseTask with a GooseTaskSet, where it is stored in the GooseTaskSet.tasks vector. The
    /// function associated with the task will be run during the load test.
    /// 
    /// # Example
    /// ```rust
    ///     let mut example_tasks = GooseTaskSet::new("ExampleTasks");
    ///     example_tasks.register_task(GooseTask::new());
    /// ```
    /// 
    /// Note: this example isn't particularly useful. It's required you also set a function on your task,
    /// otherwise your loadtest will refuse to run. See `set_function`.
    pub fn register_task(&mut self, mut task: GooseTask) {
        trace!("{} register_task: {}", self.name, task.name);
        task.tasks_index = self.tasks.len();
        self.tasks.push(task);
    }

    /// Sets a weight on a task set. The larger the value of weight, the more often the task set will 
    /// be assigned to clients. For example, if you have task set foo with a weight of 3, and task set
    /// bar with a weight of 1, and you spin up a load test with 8 clients, 6 of them will be running
    /// the foo task set, and 2 will be running the bar task set.
    /// 
    /// # Example
    /// ```rust
    ///     let mut example_tasks = GooseTaskSet::new("ExampleTasks").set_weight(3);
    /// ```
    pub fn set_weight(mut self, weight: usize) -> Self {
        trace!("{} set_weight: {}", self.name, weight);
        if weight < 1 {
            error!("{} weight of {} not allowed", self.name, weight);
            std::process::exit(1);
        }
        else {
            self.weight = weight;
        }
        self
    }

    /// Set a default host for the task set. If no `--host` flag is set when running the load test, this
    /// host will be pre-pended on all requests. For example, this can configure your load test to run
    /// against your local development environment by default, and the `--host` option could be used to
    /// override host when running the load test against production.
    /// 
    /// # Example
    /// ```rust
    ///     let mut example_tasks = GooseTaskSet::new("ExampleTasks").set_host("http://10.1.1.42");
    /// ```
    pub fn set_host(mut self, host: &str) -> Self {
        trace!("{} set_host: {}", self.name, host);
        // Host validation happens in main() at startup.
        self.host = Some(host.to_string());
        self
    }

    /// Configure a task_set to to pause after running each task. The length of the pause will be randomly
    /// selected from `min_weight` to `max_wait` inclusively.  For example, if `min_wait` is `0` and
    /// `max_weight` is `2`, the client will randomly sleep for 0, 1 or 2 seconds after each task completes.
    /// 
    /// # Example
    /// ```rust
    ///     let mut example_tasks = GooseTaskSet::new("ExampleTasks").set_wait_time(0, 1);
    /// ```
    pub fn set_wait_time(mut self, min_wait: usize, max_wait: usize) -> Self {
        trace!("{} set_wait time: min: {} max: {}", self.name, min_wait, max_wait);
        if min_wait > max_wait {
            error!("min_wait({}) can't be larger than max_weight({})", min_wait, max_wait);
            std::process::exit(1);
        }
        self.min_wait = min_wait;
        self.max_wait = max_wait;
        self
    }
}

/// Tracks the current run-mode of a client.
#[derive(Debug, Clone)]
pub enum GooseClientMode {
    /// Clients are briefly in the INIT mode when first allocated.
    INIT,
    /// Clients are briefly in the HATCHING mode when setting things up.
    HATCHING,
    /// Clients spend most of their time in the RUNNING mode, executing tasks.
    RUNNING,
    /// Clients are briefly in the EXITING mode when stopping.
    EXITING,
}

/// Commands sent between the parent and client threads.
#[derive(Debug, Clone)]
pub enum GooseClientCommand {
    /// Tell client thread to push statistics to parent
    SYNC,
    /// Tell client thread to exit
    EXIT,
}

/// Statistics collected about a path-method pair, (for example `/index`-`GET`).
#[derive(Debug, Clone)]
pub struct GooseRequest {
    /// The path for which statistics are being collected.
    pub path: String,
    /// The method for which statistics are being collected.
    pub method: Method,
    /// A record of every response time for every request of this path-method pair.
    pub response_times: Vec<f32>,
    /// Per-status-code counters, tracking how often each response code was returned for this request.
    pub status_code_counts: HashMap<u16, usize>,
    /// Total number of times this path-method request resulted in a successful (2xx) status code.
    pub success_count: usize,
    /// Total number of times this path-method request resulted in a non-successful (non-2xx) status code.
    pub fail_count: usize,
}
impl GooseRequest {
    /// Create a new GooseRequest object.
    fn new(path: &str, method: Method) -> Self {
        trace!("new request");
        GooseRequest {
            path: path.to_string(),
            method: method,
            response_times: Vec::new(),
            status_code_counts: HashMap::new(),
            success_count: 0,
            fail_count: 0,
        }
    }

    /// Append response time to `response_times` vector.
    fn set_response_time(&mut self, response_time: f32) {
        self.response_times.push(response_time);
    }

    /// Increment counter for status code, creating new counter if first time seeing status code.
    fn set_status_code(&mut self, status_code: Option<StatusCode>) {
        let status_code_u16 = match status_code {
            Some(s) => s.as_u16(),
            _ => 0,
        };
        let counter = match self.status_code_counts.get(&status_code_u16) {
            // We've seen this status code before, increment counter.
            Some(c) => {
                debug!("got {:?} counter: {}", status_code, c);
                *c + 1
            }
            // First time we've seen this status code, initialize counter.
            None => {
                debug!("no match for counter: {}", status_code_u16);
                1
            }
        };
        self.status_code_counts.insert(status_code_u16, counter);
        debug!("incremented {} counter: {}", status_code_u16, counter);
    }
}

/// An individual client state, repeatedly running all GooseTasks in a specific GooseTaskSet.
#[derive(Debug, Clone)]
pub struct GooseClient {
    /// An index into the GooseTest.task_sets vector, indicating which GooseTaskSet is running.
    pub task_sets_index: usize,
    /// A [`reqwest.blocking.client`](reqwest/*/reqwest/blocking/struct.Client.html) instance (@TODO: async).
    pub client: Client,
    /// The GooseTest.host.
    pub default_host: Option<String>,
    /// The GooseTaskSet.host.
    pub task_set_host: Option<String>,
    /// Minimum amount of time to sleep after running a task.
    pub min_wait: usize,
    /// Maximum amount of time to sleep after running a task.
    pub max_wait: usize,
    /// A local copy of the global GooseConfiguration.
    pub config: GooseConfiguration,
    /// An index into GooseTest.weighted_clients, indicating which weighted GooseTaskSet is running.
    pub weighted_clients_index: usize,
    /// The current run mode of this client, see `enum GooseClientMode`.
    pub mode: GooseClientMode,
    /// A weighted list of all tasks that run on_start.
    pub weighted_on_start_tasks: Vec<Vec<usize>>,
    /// A weighted list of all tasks that this client runs once started.
    pub weighted_tasks: Vec<Vec<usize>>,
    /// A pointer into which sequenced bucket the client is currently running tasks from.
    pub weighted_bucket: usize,
    /// A pointer of which task within the current sequenced bucket is currently running.
    pub weighted_bucket_position: usize,
    /// A weighted list of all tasks that run on_stop.
    pub weighted_on_stop_tasks: Vec<Vec<usize>>,
    /// Optional name of all requests made within the current task.
    pub request_name: String,
    /// Optional statistics collected about all requests made by this client.
    pub requests: HashMap<String, GooseRequest>,
}
impl GooseClient {
    /// Create a new client state.
    pub fn new(counter: usize, task_sets_index: usize, default_host: Option<String>, task_set_host: Option<String>, min_wait: usize, max_wait: usize, configuration: &GooseConfiguration) -> Self {
        trace!("new client");
        let builder = Client::builder()
            .user_agent(APP_USER_AGENT);
        let client = match builder.build() {
            Ok(c) => c,
            Err(e) => {
                error!("failed to build client {} for task {}: {}", counter, task_sets_index, e);
                std::process::exit(1);
            }
        };
        GooseClient {
            task_sets_index: task_sets_index,
            default_host: default_host,
            task_set_host: task_set_host,
            client: client,
            config: configuration.clone(),
            min_wait: min_wait,
            max_wait: max_wait,
            // A value of max_value() indicates this client isn't fully initialized yet.
            weighted_clients_index: usize::max_value(),
            mode: GooseClientMode::INIT,
            weighted_on_start_tasks: Vec::new(),
            weighted_tasks: Vec::new(),
            weighted_bucket: 0,
            weighted_bucket_position: 0,
            weighted_on_stop_tasks: Vec::new(),
            request_name: "".to_string(),
            requests: HashMap::new(),
        }
    }

    /// Sets the current run mode of this client.
    pub fn set_mode(&mut self, mode: GooseClientMode) {
        self.mode = mode;
    }

    /// Checks if the current path-method pair has been requested before.
    fn get_request(&mut self, path: &str, method: &Method) -> GooseRequest {
        let key = format!("{:?} {}", method, path);
        trace!("get key: {}", &key);
        match self.requests.get(&key) {
            Some(r) => r.clone(),
            None => GooseRequest::new(path, method.clone()),
        }
    }

    /// Stores request statistics about the current path-method pair.
    fn set_request(&mut self, path: &str, method: &Method, request: GooseRequest) {
        let key = format!("{:?} {}", method, path);
        trace!("set key: {}", &key);
        self.requests.insert(key, request.clone());
    }

    /// A helper that pre-pends a hostname to a path. For example, if you pass in `/foo`
    /// and `--host` is set to `http://127.0.0.1` it will return `http://127.0.0.1/foo`.
    /// Respects per-GooseTaskSet host configuration, GooseTest host configuration, and
    /// `--host` CLI configuration option.
    /// 
    /// If `path` is passed in with a hard-coded host, this will be used.
    /// 
    /// Host is defined in the following order:
    ///  - If `path` includes the host, use this
    ///  - Otherwise, if `--host` is defined, use this
    ///  - Otherwise, if `GooseTaskSet.host` is defined, use this
    ///  - Otherwise, use `GooseTest.host`.
    pub fn build_url(&mut self, path: &str) -> String {
        // If URL includes a host, use it.
        if let Ok(parsed_path) = Url::parse(path) {
            if let Some(uri) = parsed_path.host() {
                return uri.to_string()
            }
        }

        let base_url;
        // If the `--host` CLI option is set, use it to build the URL
        if self.config.host.len() > 0 {
            base_url = Url::parse(&self.config.host).unwrap();
        }
        else {
            base_url = match &self.task_set_host {
                // Otherwise, if `GooseTaskSet.host` is defined, usee this
                Some(host) => Url::parse(host).unwrap(),
                // Otherwise, use `GooseTest.host`. `unwrap` okay as host validation was done at startup.
                None => Url::parse(&self.default_host.clone().unwrap()).unwrap(),
            };
        }
        match base_url.join(path) {
            Ok(url) => url.to_string(),
            Err(e) => {
                error!("failed to build url from base {} and path {} for task {}: {}", &base_url, &path, self.task_sets_index, e);
                std::process::exit(1);
            }
        }
    }

    /// A helper to make a `GET` request of a path and collect relevant statistics.
    /// Automatically prepends the correct host.
    /// 
    /// (If you need to set headers, change timeouts, or otherwise make use of the
    /// [`reqwest::RequestBuilder`](reqwest/0.10.4/reqwest/struct.RequestBuilder.html)
    /// object, you can instead call `goose_get` which returns a RequestBuilder, then
    /// call `goose_send` to invoke the request.)
    /// 
    /// # Example
    /// ```rust
    ///     let _response = client.get("/path/to/foo");
    /// ```
    pub fn get(&mut self, path: &str) -> Result<Response, Error> {
        let request_builder = self.goose_get(path);
        let response = self.goose_send(request_builder);
        response
    }

    /// A helper to make a `POST` request of a path and collect relevant statistics.
    /// Automatically prepends the correct host.
    /// 
    /// (If you need to set headers, change timeouts, or otherwise make use of the
    /// [`reqwest::RequestBuilder`](reqwest/0.10.4/reqwest/struct.RequestBuilder.html)
    /// object, you can instead call `goose_post` which returns a RequestBuilder, then
    /// call `goose_send` to invoke the request.)
    /// 
    /// # Example
    /// ```rust
    ///     let _response = client.post("/path/to/foo", "BODY BEING POSTED");
    /// ```
    pub fn post(&mut self, path: &str, body: String) -> Result<Response, Error> {
        let request_builder = self.goose_post(path).body(body);
        let response = self.goose_send(request_builder);
        response
    }

    /// A helper to make a `HEAD` request of a path and collect relevant statistics.
    /// Automatically prepends the correct host.
    /// 
    /// (If you need to set headers, change timeouts, or otherwise make use of the
    /// [`reqwest::RequestBuilder`](reqwest/0.10.4/reqwest/struct.RequestBuilder.html)
    /// object, you can instead call `goose_head` which returns a RequestBuilder, then
    /// call `goose_send` to invoke the request.)
    /// 
    /// # Example
    /// ```rust
    ///     let _response = client.head("/path/to/foo");
    /// ```
    pub fn head(&mut self, path: &str) -> Result<Response, Error> {
        let request_builder = self.goose_head(path);
        let response = self.goose_send(request_builder);
        response
    }

    /// A helper to make a `DELETE` request of a path and collect relevant statistics.
    /// Automatically prepends the correct host.
    /// 
    /// (If you need to set headers, change timeouts, or otherwise make use of the
    /// [`reqwest::RequestBuilder`](reqwest/0.10.4/reqwest/struct.RequestBuilder.html)
    /// object, you can instead call `goose_delete` which returns a RequestBuilder,
    /// then call `goose_send` to invoke the request.)
    /// 
    /// # Example
    /// ```rust
    ///     let _response = client.delete("/path/to/foo");
    /// ```
    pub fn delete(&mut self, path: &str) -> Result<Response, Error> {
        let request_builder = self.goose_delete(path);
        let response = self.goose_send(request_builder);
        response
    }

    /// Prepends the correct host on the path, then prepares a
    /// [`reqwest::RequestBuilder`](reqwest/0.10.4/reqwest/struct.RequestBuilder.html)
    /// object for making a `GET` request.
    /// 
    /// (You must then call `goose_send` on this object to actually execute the request.)
    /// 
    /// # Example
    /// ```rust
    ///     let request_builder = client.goose_get("/path/to/foo");
    ///     let response = self.goose_send(request_builder);
    /// ```
    pub fn goose_get(&mut self, path: &str) -> RequestBuilder {
        let url = self.build_url(path);
        self.client.get(&url)
    }

    /// Prepends the correct host on the path, then prepares a
    /// [`reqwest::RequestBuilder`](reqwest/0.10.4/reqwest/struct.RequestBuilder.html)
    /// object for making a `POST` request.
    /// 
    /// (You must then call `goose_send` on this object to actually execute the request.)
    /// 
    /// # Example
    /// ```rust
    ///     let request_builder = client.goose_post("/path/to/foo");
    ///     let response = self.goose_send(request_builder);
    /// ```
    pub fn goose_post(&mut self, path: &str) -> RequestBuilder {
        let url = self.build_url(path);
        self.client.post(&url)
    }

    /// Prepends the correct host on the path, then prepares a
    /// [`reqwest::RequestBuilder`](reqwest/0.10.4/reqwest/struct.RequestBuilder.html)
    /// object for making a `HEAD` request.
    /// 
    /// (You must then call `goose_send` on this object to actually execute the request.)
    /// 
    /// # Example
    /// ```rust
    ///     let request_builder = client.goose_head("/path/to/foo");
    ///     let response = self.goose_send(request_builder);
    /// ```
    pub fn goose_head(&mut self, path: &str) -> RequestBuilder {
        let url = self.build_url(path);
        self.client.head(&url)
    }

    /// Prepends the correct host on the path, then prepares a
    /// [`reqwest::RequestBuilder`](reqwest/0.10.4/reqwest/struct.RequestBuilder.html)
    /// object for making a `PUT` request.
    /// 
    /// (You must then call `goose_send` on this object to actually execute the request.)
    /// 
    /// # Example
    /// ```rust
    ///     let request_builder = client.goose_put("/login");
    /// ```
    pub fn goose_put(&mut self, path: &str) -> RequestBuilder {
        let url = self.build_url(path);
        self.client.put(&url)
    }

    /// Prepends the correct host on the path, then prepares a
    /// [`reqwest::RequestBuilder`](reqwest/0.10.4/reqwest/struct.RequestBuilder.html)
    /// object for making a `PATCH` request.
    /// 
    /// (You must then call `goose_send` on this object to actually execute the request.)
    /// 
    /// # Example
    /// ```rust
    ///     let request_builder = client.goose_patch("/path/to/foo");
    /// ```
    pub fn goose_patch(&mut self, path: &str) -> RequestBuilder {
        let url = self.build_url(path);
        self.client.patch(&url)
    }

    /// Prepends the correct host on the path, then prepares a
    /// [`reqwest::RequestBuilder`](reqwest/0.10.4/reqwest/struct.RequestBuilder.html)
    /// object for making a `DELETE` request.
    /// 
    /// (You must then call `goose_send` on this object to actually execute the request.)
    /// 
    /// # Example
    /// ```rust
    ///     let request_builder = client.goose_delete("/path/to/foo");
    /// ```
    pub fn goose_delete(&mut self, path: &str) -> RequestBuilder {
        let url = self.build_url(path);
        self.client.delete(&url)
    }

    /// Builds the provided
    /// [`reqwest::RequestBuilder`](reqwest/0.10.4/reqwest/struct.RequestBuilder.html)
    /// object and then executes the response. If statistics are being displayed, it
    /// also captures request statistics.
    /// 
    /// It is possible to build and execute a `RequestBuilder` object without using this
    /// helper function, but then no statistics will be captured.
    /// 
    /// # Example
    /// ```rust
    ///     let request_builder = client.goose_get("/path/to/foo");
    ///     let response = self.goose_send(request_builder);
    /// ```
    pub fn goose_send(&mut self, request_builder: RequestBuilder) -> Result<Response, Error> {
        let started = Instant::now();
        let request = request_builder.build()?;

        // Allow introspection.
        let method = request.method().clone();
        let url = request.url().to_string();

        // Make the actual request.
        let response = self.client.execute(request);
        let elapsed = started.elapsed() * 100;

        if self.config.print_stats {
            // Introspect the request for logging and statistics
            let path = match Url::parse(&url) {
                Ok(u) => u.path().to_string(),
                Err(e) => {
                    warn!("failed to parse url: {}", e);
                    "parse error".to_string()
                }
            };
            // By default requests are recorded as "METHOD URL", allow override of "METHOD NAME"
            let request_name;
            if self.request_name != "" {
                request_name = self.request_name.to_string();
            }
            else {
                request_name = path.to_string();
            }
            let mut goose_request = self.get_request(&request_name, &method.clone());
            goose_request.set_response_time(elapsed.as_secs_f32());
            match &response {
                Ok(r) => {
                    let status_code = r.status();
                    // Only increment status_code_counts if we're displaying the results
                    if self.config.status_codes {
                        goose_request.set_status_code(Some(status_code));
                    }

                    debug!("{:?}: status_code {}", &path, status_code);
                    // @TODO: match/handle all is_foo() https://docs.rs/http/0.2.1/http/status/struct.StatusCode.html
                    if status_code.is_success() {
                        goose_request.success_count += 1;
                    }
                    // @TODO: properly track redirects and other code ranges
                    else {
                        // @TODO: handle this correctly
                        warn!("{:?}: non-success status_code: {:?}", &path, status_code);
                        goose_request.fail_count += 1;
                    }
                }
                Err(e) => {
                    // @TODO: what can we learn from a reqwest error?
                    warn!("{:?}: {}", &path, e);
                    goose_request.fail_count += 1;
                    if self.config.status_codes {
                        goose_request.set_status_code(None);
                    }
                }
            };
            self.set_request(&request_name, &method, goose_request);
        }
        response
    }
}

/// An individual task within a `GooseTaskSet`.
#[derive(Clone)]
pub struct GooseTask {
    // This is the GooseTaskSet.tasks index
    pub tasks_index: usize,
    pub name: String,
    pub weight: usize,
    pub sequence: usize,
    pub on_start: bool,
    pub on_stop: bool,
    pub function: Option<fn(&mut GooseClient)>,
}
impl GooseTask {
    pub fn new() -> Self {
        trace!("new task");
        let task = GooseTask {
            tasks_index: usize::max_value(),
            name: "".to_string(),
            weight: 1,
            sequence: 0,
            on_start: false,
            on_stop: false,
            function: None,
        };
        task
    }

    pub fn named(name: &str) -> Self {
        trace!("new task: {}", name);
        let task = GooseTask {
            tasks_index: usize::max_value(),
            name: name.to_string(),
            weight: 1,
            sequence: 0,
            on_start: false,
            on_stop: false,
            function: None,
        };
        task
    }

    pub fn set_on_start(mut self) -> Self {
        trace!("{} [{}] set_on_start task", self.name, self.tasks_index);
        self.on_start = true;
        self
    }

    pub fn set_on_stop(mut self) -> Self {
        trace!("{} [{}] set_on_stop task", self.name, self.tasks_index);
        self.on_stop = true;
        self
    }

    pub fn set_name(mut self, name: &str) -> Self {
        trace!("[{}] set_name: {}", self.tasks_index, self.name);
        self.name = name.to_string();
        self
    }

    /// Sets a weight on an individual task. The larger the value of weight, the more often it will be run
    /// in the TaskSet. For example, if one task has a weight of 3 and another task has a weight of 1, the
    /// first task will run 3 times as often.
    /// 
    /// # Example
    /// ```rust
    ///     let a_task = GooseTask::new().set_weight(3);
    /// ```
    pub fn set_weight(mut self, weight: usize) -> Self {
        trace!("{} [{}] set_weight: {}", self.name, self.tasks_index, weight);
        if weight < 1 {
            error!("{} weight of {} not allowed", self.name, weight);
            std::process::exit(1);
        }
        else {
            self.weight = weight;
        }
        self
    }

    pub fn set_sequence(mut self, sequence: usize) -> Self {
        trace!("{} [{}] set_sequence: {}", self.name, self.tasks_index, sequence);
        if sequence < 1 {
            info!("setting sequence to 0 for task {} is unnecessary, sequence disabled", self.name);
        }
        self.sequence = sequence;
        self
    }

    pub fn set_function(mut self, function: fn(&mut GooseClient)) -> Self {
        self.function = Some(function);
        self
    }
}
