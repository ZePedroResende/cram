use crossbeam::Sender;
use std::collections::HashMap;
use crossbeam::Receiver;
use queue::Queue;

pub mod simple;
pub mod label;


pub struct Label {

    input_channel: Receiver<Vec<u8>>,    
    
    map : HashMap<String, Option<Box<FnMut(Vec<u8>) + Send + Sync + 'static>>>,
    
    default_fun : Option<Box<FnMut(Vec<u8>) + Send +  Sync + 'static>>,

    fn_receiver : Receiver< (Option<String>,Box<FnMut(Vec<u8>) + Send + Sync + 'static>)>,
    
    fn_sender : Sender< (Option<String>,Box<FnMut(Vec<u8>) + Send + Sync + 'static>) >,

    queue_by_label : HashMap< Option<String>, Queue< Vec<u8> > >,

}

pub struct Simple {

    input_channel: Receiver<Vec<u8>>,

    handler : Box<FnMut(Vec<u8>) + Send + Sync + 'static>,
    
}