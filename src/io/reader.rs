
use std::sync::{Arc, Mutex};

pub struct Reader{
    pull_socket : zmq::Socket,
}

impl Reader{
    
    pub fn new( context : Arc<Mutex<zmq::Context>>, port : i32) -> Reader {

        let pull_socket = context.lock().unwrap().socket(zmq::PULL).unwrap();
        
        pull_socket.bind( &format!("tcp://*:{}", port)).unwrap();

        Reader{
            pull_socket : pull_socket,
        }
    }

    pub fn read(&self) -> Vec<u8>{
        self.pull_socket.recv_msg(0).unwrap().to_vec()    
    }
    
}