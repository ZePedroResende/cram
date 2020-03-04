use crossbeam::Receiver;
use std::collections::HashMap;
use std::sync::Arc;

pub mod label;
pub mod simple;

pub struct Label {
    input_channel: Receiver<Vec<u8>>,

    map: HashMap<String, Arc<Box<Fn(Vec<u8>) + Send + Sync + 'static>>>,

    default_fun: Arc<Box<Fn(Vec<u8>) + Send + Sync + 'static>>,
}

pub struct Simple {
    input_channel: Receiver<(Vec<u8>)>,

    handler: Arc<Box<Fn(Vec<u8>) + Send + Sync + 'static>>,
}
