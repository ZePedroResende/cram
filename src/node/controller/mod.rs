use std::boxed::Box;
use std::collections::HashMap;
use std::ops::Fn;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::node::io::*;
use crate::serializers::*;

pub struct Controller<'a> {
    io: Arc<Mutex<Io>>,
    map: Arc<Mutex<HashMap<String, Box<Fn(&Vec<u8>) + 'a>>>>,
}

impl<'a> Controller<'a> {
    pub fn new() -> Controller<'a> {
        Controller {
            io: Arc::new(Mutex::new(Io::new(12345))),
            map: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn register_handler<F: Fn(&Vec<u8>) + 'a>(&mut self, label: &str, f: F) {
        let mut map_lock = self.map.lock().unwrap();

        map_lock.insert(String::from(label), Box::new(f));
    }

    pub fn start(self) {}

    pub fn handle(&self, msg: (Vec<u8>, i8)) {}

    /*
    pub fn add_connection(self, address: String) {
        let io_lock = self.io.lock().unwrap();
        io_lock.add_connection(address);
    }

    pub fn broadcast(self, msg: Vec<u8>, label: String) {
        let mut bytes = serialize_label_message(&label, &msg);
        put_type(1, &mut bytes);
        let io_lock = self.io.lock().unwrap();
        io_lock.broadcast(bytes);
    }
    */
}
