use std::boxed::Box;
use std::collections::HashMap;
use std::ops::Fn;
use std::sync::{Arc, Mutex};
use std::thread;
use std::vec::Vec;

use super::Controller;
use crate::serializers::*;

pub struct LabelController {
    default_handler: Arc<Mutex<Box<Fn(&Vec<u8>) + 'static>>>,

    map: Arc<Mutex<HashMap<String, Box<Fn(&Vec<u8>) + 'static>>>>,

    writer: spmc::Sender<(Vec<u8>, String)>,
    reader: spmc::Receiver<(Vec<u8>, String)>,
}

impl LabelController {
    fn new<F: Fn(&Vec<u8>) + 'static>(fun: F) -> LabelController {
        let (s, r) = spmc::channel();

        LabelController {
            default_handler: Arc::new(Mutex::new(Box::new(fun))),
            map: Arc::new(Mutex::new(HashMap::new())),
            writer: s,
            reader: r,
        }
    }

    fn reply(&self, message: Vec<u8>, label: String, address: String) {
        let bytes = serialize_label_message(&label, &message);

        self.writer.send((bytes, address)).unwrap();
    }

    fn add_handlers<F: Fn(&Vec<u8>) + 'static>(&self, label: String, fun: F) {
        let mut my_map = self.map.lock().unwrap();

        my_map.insert(label, Box::new(fun));
    }
}

impl Controller for LabelController {
    fn call(&self, message: &Vec<u8>) {
        let (label, real_message) = deserialize_label_message(message);

        match self.map.lock().unwrap().get(&label) {
            Some(handler) => handler(&real_message),

            None => {
                let f = self.default_handler.lock().unwrap();

                f(&real_message);
            }
        };
    }

    fn get_message(&self) -> (Vec<u8>, String) {
        self.reader.recv().unwrap()
    }
}
