mod io;
mod thread_pool;
mod controller;
pub mod node;
pub mod builder;

use controller::stateless;
use controller::stateful;
use io::Io;

use std::collections::HashMap;
use crossbeam::crossbeam_channel::Sender;

pub struct Builder{
    io : Io,
    
    simple_controller : Option<stateless::Simple>,
    
    simple_controller_mut : Option<stateful::Simple>,
    
    label_controller : Option<stateless::Label>,

    label_controller_mut : Option<stateful::Label>,
    
    list : Vec< (String, Box<Fn(Vec<u8>) + Send + Sync + 'static>)>,
    
    list_mut : Vec< (String, Box<FnMut(Vec<u8>) + Send + Sync + 'static>)>,

    configuration : Node,

    has_controllers : bool,
}


pub struct Node{
    io :  Sender<( i8, Vec<u8>, String) >,
    io_port : usize,
    controllers : HashMap< i8, Sender< Vec<u8> > >,
}

