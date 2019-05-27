use crossbeam::Receiver;
use std::sync::Arc;
use std::collections::HashMap;

pub mod simple;
pub mod label;


pub struct Label {
   
    input_channel: Receiver<Vec<u8>>,    
    
    map : HashMap<String, Arc<Box<Fn(Vec<u8>) + Send + Sync + 'static>>>,
    
    default_fun : Arc<Box<Fn(Vec<u8>) + Send +  Sync + 'static>>,
}

pub struct Simple {
 
    input_channel: Receiver<(Vec<u8>)>,
 
    handler : Arc<Box<Fn(Vec<u8>) + Send + Sync + 'static>>,
}