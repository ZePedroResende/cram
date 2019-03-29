mod controller;
mod io;
use controller::Controller;
use io::Io;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Node<'a> {
    io: Io,
    controllers: HashMap<i8, Controller<'a>>,
}

impl<'a> Node<'a> {
    fn new(port: i32) -> Node<'a> {
        Node {
            io: Io::new(port),
            controllers: HashMap::new(),
        }
    }

    fn add_contoller(&mut self, type_flag: i8, controller: Controller<'a>) {
        self.controllers.insert(type_flag, controller);
    }

    fn start(&mut self) {
        let msg = self.io.read();
        self.controllers.get(&msg.1).unwrap().handle(msg);
    }
}
