#![allow(dead_code)]

mod dns;

use crate::node::Node;
use crossbeam::Receiver;
use dns::Dns;
use std::collections::HashMap;
use std::thread;

use crate::serializers::*;

pub struct Io {
    input_channel: Receiver<(i8, Vec<u8>, String)>,
    port: usize,
    option_dns: Option<Dns>,
}

impl Io {
    pub fn new(port: usize, input_channel: Receiver<(i8, Vec<u8>, String)>) -> Io {
        Io {
            input_channel: input_channel,
            port: port,
            option_dns: None,
        }
    }

    pub fn new_with_dns(
        port: usize,
        input_channel: Receiver<(i8, Vec<u8>, String)>,
        dns_path: String,
    ) -> Io {
        Io {
            input_channel: input_channel,
            port: port,
            option_dns: Some(Dns::new(dns_path)),
        }
    }

    pub fn start(mut self, node: Node) {
        let mut context = zmq::Context::new();

        let pull_socket = context.socket(zmq::PULL).unwrap();
        pull_socket.bind(&format!("tcp://*:{}", self.port)).unwrap();

        let _read_from_sck = thread::spawn(move || loop {
            let mut vec;
            match pull_socket.recv_bytes(0) {
                Ok(val) => vec = val,
                Err(_e) => break,
            }

            let message_type = get_type(&mut vec);

            match node.controllers.get(&message_type) {
                None => (),
                Some(c) => c.send(vec).unwrap(),
            };
        });

        let _write_to_sck = thread::spawn(move || {
            let mut push_sockets: HashMap<String, zmq::Socket> = HashMap::new();

            loop {
                let (controller_type, mut vec, mut to);
                match self.input_channel.recv() {
                    Ok((v1, v2, v3)) => {
                        controller_type = v1;
                        vec = v2;
                        to = v3;
                    }
                    Err(_e) => {
                        context.destroy().unwrap();
                        break;
                    }
                }

                put_type(controller_type, &mut vec);

                self.option_dns = match self.option_dns {
                    None => None,
                    Some(dns) => {
                        match dns.get_address(&to) {
                            Some(addr) => to = addr,
                            None => (),
                        };
                        Some(dns)
                    }
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
