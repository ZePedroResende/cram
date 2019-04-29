pub mod controller;
pub mod io;
pub mod dispatcher;

use controller::Controller;
use io::Io;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};


pub struct Node {
}

impl Node {
    pub fn new(_port: i32) -> Node {
        Node {
        }
    }    
}
