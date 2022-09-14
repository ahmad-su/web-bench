//function to take arguments from terminal
pub fn get_args() -> (String, String, String, usize, usize) {
    use std::env;

    //A buffer to store those arguments
    let mut args: Vec<String> = env::args().collect();
    //Make sure the user give all needed arguments
    assert!(args.len() == 6);
    //store test & worker counts; panic when user didn't satisfy it
    let counts:usize = args.pop().unwrap().trim().parse().expect("Parse Error: Couldn't take [test_counts] field from arguments. Please give a correct number");
    let worker: usize = args.pop().unwrap().trim().parse().expect(
        "Parse Error: Couldn't take [worker] field from arguments. Please give a correct number",
    );
    //get the rest fields
    let path = args.pop().unwrap();
    let method = args.pop().unwrap();
    let addr = args.pop().unwrap();
    //destroy the buffer
    drop(args);

    //give message to user about the test
    if worker != 1 {println!(
        "Testing [with {counts} requests @ {worker} workers] for:\n[Target: {addr}] [Method: {method}] [Path: {path}]\n"
    );} else {println!(
        "Testing [with {counts} requests @ {worker} worker] for:\n[Target: {addr}] [Method: {method}] [Path: {path}]\n"
    );}

    //return as tuple
    (addr, method, path, worker, counts)
}

//This config struct is used to store user input (arguments) from terminal
//All its fields are public to anyone who takes or borrow it.
#[derive(Clone)]
pub struct Config {
    pub addr: String,
    pub method: String,
    pub path: String,
    pub worker: usize,
    pub counts: usize,
}

impl Config {
    pub fn new(addr: String, method: String, path: String, worker: usize, counts: usize) -> Self {
        Self {
            addr,
            method,
            path,
            worker,
            counts,
        }
    }
}

