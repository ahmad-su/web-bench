pub mod cfg;
pub mod task;

use cfg::Config;

use std::{
    sync::{
        mpsc::{self, Sender},
        Arc, Mutex,
    },
    thread, time::SystemTime,
};

//Define object trait that will be the task for each threads to finish
type Task = Box<dyn FnOnce() + Send + 'static>;

 //App Engine is where the main logic placed
//This struct will spawn threads, give task to each threads, watch & save the time before and after the tasks end
pub struct AppEngine {
    pub threads: Vec<Option<thread::JoinHandle<()>>>,
    pub sender: Option<Sender<Task>>,
    pub time: Arc<Mutex<u128>>,
    counts: u128,
}

impl AppEngine {
    pub fn new(cfg: &Config) -> Self {
        //Create sender & receiver instances
        //we wrap the receiver inside Arc<Mutex<>>
        //The arc allows us to share the object between threads
        //And the mutex allows us to prevents collision when the threads are trying to access the receiver
        //Although it's shared, Only one thread allowed to have access at a time
        let (sender, receiver) = mpsc::channel();
        let sender: Sender<Task> = sender;
        //we wrap the sender inside Option<T> 
        //because we will drop it later when all the tasks are done
        let sender = Some(sender);
        let receiver = Arc::new(Mutex::new(receiver));

        //The time field is also shared between threads
        //So each threads could modify it
        let time = Arc::new(Mutex::new(0));

        //Create vector to store all threads
        let mut threads = Vec::new();
        for id in 0..cfg.worker {
            //clone the receiver's reference for every threads
            let recv = receiver.clone();
            //clone the time's reference for every threads
            let thread_time = time.clone();
            //spawn and store threads into the vec
            //panic when the main threads unable to spawn thread
            threads.push(
                Some(thread::Builder::new()
                    .name(format!("Thread[{}]", id))
                    .spawn(move || 
                        {
                            //define local task counter
                            let mut count:usize = 0;
                            //Create local time field to store each thread's working time
                            let start_time = SystemTime::now();
                            loop {
                                match recv.lock().unwrap().recv() {
                                    Ok(task) => {
                                        count +=1;
                                        //uncomment for testing purposes
                                        //println!("Thread[{id}] got a task [{count}], executing..");
                                        task();
                                    }
                                    Err(_) => {
                                        if count != 0 {println!("Thread[{id}] finished all task, shutting down... Tasks done: [{count}]");}
                                        else {println!("Thread[{id}] didn't receive any task. Tasks done: [{count}]");}
                                        *thread_time.lock().unwrap() = start_time.elapsed().unwrap().as_millis();
                                        break ();
                                    },
                                }
                            }
                        }
                        )
                    .unwrap(),
            )
        )
        }
        //let threads = Arc::new(Mutex::new(threads));
        let counts = cfg.counts as u128;
        Self { threads, sender, time, counts }
    }
    // pub fn get_time(&self) -> u128 {
    //     let time_ref = self.time.lock().unwrap();
    //     *time_ref
    // }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let task = Box::new(f);
        self.sender.as_ref().unwrap().send(task).unwrap();
    }
}

impl Drop for AppEngine {
    //behavior when all tasks have been finished 
    fn drop(&mut self) {
        //drop sender
        drop(self.sender.take());

        //make all threads join the main thread
        while let Some(thread) = &mut self.threads.pop() {
            //another optional print to stdout
            //println!("Shutting down worker..",);

            //take out Join handler from option then call join() to join main
            if let Some(thread) = thread.take() {
                thread.join().unwrap();
                
            }            
        }

        //Get lock to time field; print time when all tasks finished; including its speed;
        let time = self.time.lock().unwrap();
        println!(
            "All Threads finished tasks at {}ms. Rate: {}reqs/s\nContribute: https://github.com/ahmad-su/web-bench",*time, self.counts * 1000/ *time);
    }
}
