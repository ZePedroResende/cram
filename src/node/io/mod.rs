mod dns;

use crate::node::configuration::Configuration;
use crossbeam::Receiver;
use std::collections::HashMap;
use std::thread;
use dns::Dns;

use crate::serializers::*;

pub struct Io { 
    input_channel : Receiver<(i8,Vec<u8>,String)>,
    port : usize,
    dns : Dns,
}

impl Io {

    pub fn new(port: usize, input_channel: Receiver<(i8,Vec<u8>,String)> ) -> Io {
        
        Io {
            input_channel : input_channel,
            port : port,
            dns : Dns::new(),
        }
    }

    pub fn start(self, config : Configuration){

        let context = zmq::Context::new();
                
        let pull_socket = context.socket(zmq::PULL).unwrap();
        
        pull_socket.bind(&format!("tcp://*:{}", self.port)).unwrap();
        
        let _read_thread = thread::spawn(move || loop {

            let mut vec = pull_socket.recv_msg(0).unwrap().to_vec();

            let message_type = get_type(&mut vec);
            
            // falta ir buscar o sitio onde veio a mnesagem
            // e traduzir o nome 
            
            match config.controllers.get(&message_type){
                None => (),
                Some(c) => c.send( vec ).unwrap(),
            };
        });

        let _write_thread = thread::spawn(move || {
            
            let mut push_sockets : HashMap<String, zmq::Socket> = HashMap::new();

            loop {
                let (controller_type,mut vec, mut to) = self.input_channel.recv().unwrap();

                put_type( controller_type, &mut vec);

                match self.dns.get_address(&to) {
                    Some(addr) => 
                        to =  addr,
                    None => (),
                };

                if !push_sockets.contains_key(&to) {
                    let sck = context.socket(zmq::PUSH).unwrap();
                    sck.connect(&format!("tcp://{}", to)).unwrap();
                    push_sockets.insert(to.clone(), sck);
                }

                let sck = push_sockets.get(&to).unwrap();
                sck.send(&vec, 0).unwrap();
            }
        });
        
    }   
}
