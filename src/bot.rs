use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::{thread, time};

pub fn pull_client( port : i32 , context : Arc<Mutex<zmq::Context>>) -> JoinHandle<()>{
    
    thread::spawn( move ||{

        let pull_socket = context.lock().unwrap().socket(zmq::PULL).unwrap();
    
        pull_socket.bind( &format!("tcp://*:{}", port)).unwrap();
        
        loop {
            let msg = pull_socket.recv_msg(0).unwrap();
            let result = msg.as_str().unwrap();
            println!( "[bot][{}] Receive msg: {}", port, result);
        }
    })
}




pub fn push_client( address : String, 
                    context :  Arc<Mutex<zmq::Context>>, 
                    tag : String)  -> JoinHandle<()>{
    
    thread::spawn( move ||{
    
        let push_socket = context.lock().unwrap().socket(zmq::PUSH).unwrap();

        push_socket.connect( &format!("tcp://{}", address)).unwrap();
        
        let new_message = format!("{}||ola mundo", tag);
        
        let bytes = new_message.as_bytes();

        for _ in 0..10 {    
            push_socket.send(&bytes,0).unwrap();
            thread::sleep(time::Duration::from_millis(500));   
        }
    })
}
