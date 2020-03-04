use super::Node;
use crate::serializers;

use crossbeam::Sender;
use std::collections::HashMap;

impl Node {
    pub fn new(io_port: usize, io_channel: Sender<(i8, Vec<u8>, String)>) -> Node {
        Node {
            io: io_channel,
            io_port: io_port,
            controllers: HashMap::new(),
        }
    }

    pub fn send(&self, msg: Vec<u8>, address: String) {
        self.io.send((0, msg, address)).unwrap();
    }

    pub fn send_with_label(&self, msg: Vec<u8>, label: String, address: String) {
        let new_msg = serializers::serialize_label_message(label, msg);
        self.io.send((1, new_msg, address)).unwrap();
    }

    pub fn get_io_port(&self) -> usize {
        self.io_port
    }
}

impl Clone for Node {
    fn clone(&self) -> Node {
        let mut hash = HashMap::new();

        for (k, v) in &self.controllers {
            hash.insert(*k, v.clone());
        }

        Node {
            io: self.io.clone(),
            io_port: self.io_port,
            controllers: hash,
        }
    }
}
