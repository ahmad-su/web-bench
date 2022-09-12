use std::{net::TcpStream, io::{BufWriter, BufReader, Write, Read}};

use crate::cfg::Config;

// This is the task that will be sent to channel 
// and distributed to each threads
pub fn task(cfg : &mut Config){
    //init a TCP stream, will panic while failed instead of retrying
    let stream = TcpStream::connect(&cfg.addr[..]).unwrap();

    //create request based on config
    let req = format!("{} {} HTTP/1.1\r\n\r\n", &cfg.method[..], &cfg.path[..]);

    //writer & reader to read & write from/to the stream
    let mut writer = BufWriter::new(stream.try_clone().unwrap());
    let mut reader = BufReader::new(stream);

    //Response buffer
    let mut _response = String::new();
    
    //Send packet & receive response
    writer.write_all(req.as_bytes()).unwrap();
    writer.flush().unwrap();
    reader.read_to_string(&mut _response).unwrap();
    //uncomment to see response
    //println!("Response: {}",response);
}