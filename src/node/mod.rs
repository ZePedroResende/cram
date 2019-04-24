mod controller;
mod io;
use controller::Controller;
use io::Io;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Node {
    io: Io,
    controllers: HashMap<i8, Box<Controller + 'static>>,
}

impl Node {
    pub fn new(port: i32) -> Node {
        Node {
            io: Io::new(port),
            controllers: HashMap::new(),
        }
    }

    pub fn add_contoller<T>(&mut self, type_flag: i8, controller: Box<T>)
    where
        T: Controller + 'static,
    {
        self.controllers.insert(type_flag, controller);
    }

    pub fn start(&mut self) {
        let msg = self.io.read();
        self.controllers.get(&msg.1).unwrap().call(&msg.0);
    }
}
