use crossbeam::Receiver;
use crossbeam::Sender;
use queue::Queue;
use std::collections::HashMap;

pub mod label;
pub mod simple;

pub struct Label {
    input_channel: Receiver<Vec<u8>>,

    map: HashMap<String, Option<Box<FnMut(Vec<u8>) + Send + Sync + 'static>>>,

    default_fun: Option<Box<FnMut(Vec<u8>) + Send + Sync + 'static>>,

    fn_receiver: Receiver<(Option<String>, Box<FnMut(Vec<u8>) + Send + Sync + 'static>)>,

    fn_sender: Sender<(Option<String>, Box<FnMut(Vec<u8>) + Send + Sync + 'static>)>,

    queue_by_label: HashMap<Option<String>, Queue<Vec<u8>>>,

    pending: i32,

    stoped: bool,
}

pub struct Simple {
    input_channel: Receiver<Vec<u8>>,

    handler: Box<FnMut(Vec<u8>) + Send + Sync + 'static>,
}
