use std::time::SystemTime;
use web_bench::{
    cfg::{self, Config},
    task::task,
    AppEngine
};

fn main() {
    println!(
        "WEB-BENCH v0.1.0\nSimple Webserver Benchmark Tool. ~ By. Hendz
    USAGE: web-bench [addr:port] [method] [path] [worker] [test_counts]\n"
    );

    //get arguments from user input
    let (addr, method, path, worker, counts) = cfg::get_args();

    //Save arguments to Config
    let config = Config::new(addr, method, path, worker, counts);

    //Build App Engine using config
    let app = AppEngine::new(&config);

    //Save the time before sending task to channel (queue)
    let start_time = SystemTime::now();

    //Send the task
    for _task_number in 0..counts {
        //We clone config because every closure call will modify it
        //Don't worry; it's fast and cheap;
        let mut cfg = config.clone();
        //Send task to channel (queue)
        app.execute(move || task(&mut cfg));
    }
    //Get the time when finished sending task to channel & print it to stdout
    let end_time = start_time.elapsed().unwrap().as_millis();
    println!(
        "Finished sending [{}]tasks in [{}]ms\nWaiting tasks to finish..",
        counts, end_time
    );

    //Here, main thread has no more instruction to do
    //But the program won't end yet because the threads spawned will join the main thread
    //Main thread is waiting all threads to join when they finished their tasks
}
