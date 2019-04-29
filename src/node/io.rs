use crossbeam::Receiver;
use crossbeam::Sender;
use crossbeam::crossbeam_channel::unbounded;
use std::collections::HashMap;
use std::thread;

use crate::serializers::*;

pub struct Io {
    input_channel: Sender<(Vec<u8>, String)>,
    output_channel: Receiver<(Vec<u8>, String)>,
}

impl Io {
    pub fn new(port: i32) -> Io {
        
        let (input_s, input_r)  : (_,Receiver<(Vec<u8>, String)>)=  unbounded();
        let (output_s, output_r) =  unbounded();

        let context = zmq::Context::new();
        let pull_socket = context.socket(zmq::PULL).unwrap();
        pull_socket.bind(&format!("tcp://*:{}", port)).unwrap();
        
        let s = output_s.clone();

        let _read_thread = thread::spawn(move || loop {
            let vec = pull_socket.recv_msg(0).unwrap().to_vec();
            s.send( (vec, "unknown".to_string()) ).unwrap();
        });

        let r = input_r.clone();

        let _write_thread = thread::spawn(move || {
            let mut push_sockets : HashMap<String, zmq::Socket> = HashMap::new();
        
            loop {
                let (vec,address) = r.recv().unwrap();
            
                if !push_sockets.contains_key(&address) {
                    let sck = context.socket(zmq::PUSH).unwrap();
                    sck.connect(&format!("tcp://{}", address)).unwrap();
                    push_sockets.insert(address.clone(), sck);
                }

                let sck = push_sockets.get(&address).unwrap();
                sck.send(&vec, 0).unwrap();
            }
        });
        
        Io {
            input_channel: input_s,
            output_channel: output_r, 
        }
        
    }

    pub fn get_input_channel(&self) -> Sender<(Vec<u8>, String)> {
        self.input_channel.clone()
    }

     pub fn get_output_channel(&self) -> Receiver<(Vec<u8>, String)> {
        self.output_channel.clone()
    }
   
}
