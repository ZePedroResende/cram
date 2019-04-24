use std::boxed::Box;
use std::ops::Fn;
use std::sync::{Arc, Mutex};
use std::vec::Vec;

use super::Controller;

pub struct SimpleController<'a> {
    handler: Arc<Mutex<Box<Fn(&Vec<u8>) + 'a>>>,
    writer: spmc::Sender<(Vec<u8>, String)>,
    reader: spmc::Receiver<(Vec<u8>, String)>,
}

impl<'a> SimpleController<'a> {
    pub fn new<F: Fn(&Vec<u8>) + 'a>(fun: F) -> SimpleController<'a> {
        let (s, r) = spmc::channel();

        SimpleController {
            handler: Arc::new(Mutex::new(Box::new(fun))),
            writer: s,
            reader: r,
        }
    }

    fn reply(&self, message: Vec<u8>, address: String) {
        self.writer.send((message, address)).unwrap();
    }

    fn set_handler<F: Fn(&Vec<u8>) + 'a>(&self, fun: F) {
        let mut state = self.handler.lock().unwrap();
        *state = Box::new(fun);
    }
}

impl<'a> Controller for SimpleController<'a> {
    fn call(&self, message: &Vec<u8>) {
        let f = self.handler.lock().unwrap();
        f(message);
    }

    fn get_message(&self) -> (Vec<u8>, String) {
        self.reader.recv().unwrap()
    }
}
