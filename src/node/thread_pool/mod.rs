mod thread_message;
mod thread_pool;

use crossbeam::crossbeam_channel::Sender;
use std::boxed::Box;
use std::sync::Arc;

/// /// info to send to about stateful or stateless /// ///

struct Stateful {
    fun: Box<FnMut(Vec<u8>) + Send + Sync + 'static>,
    message: Vec<u8>,
    sender: Sender<(Option<String>, Box<FnMut(Vec<u8>) + Send + Sync + 'static>)>,
    label: Option<String>,
}

struct Stateless {
    fun: Arc<Box<Fn(Vec<u8>) + Send + Sync + 'static>>,
    message: Vec<u8>,
}

/// /// Message to send to threadPoll /// ///

pub struct ThreadMessage {
    is_stateless: bool,
    stateful_info: Option<Stateful>,
    statefless_info: Option<Stateless>,
}

pub struct ThreadPool {
    input_channel: Sender<ThreadMessage>,
}
