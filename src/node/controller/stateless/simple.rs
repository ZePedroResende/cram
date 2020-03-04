use crossbeam::{Receiver, Sender};
use std::boxed::Box;
use std::ops::Fn;
use std::sync::Arc;
use std::thread;
use std::vec::Vec;

use crate::node::thread_pool::ThreadMessage;

use super::Simple;

impl Simple {
    pub fn new<F>(fun: F, input_channel: Receiver<Vec<u8>>) -> Simple
    where
        F: Fn(Vec<u8>) + Send + Sync + 'static,
    {
        Simple {
            input_channel: input_channel,
            handler: Arc::new(Box::new(fun)),
        }
    }

    pub fn start(self, s: Sender<ThreadMessage>) {
        thread::spawn(move || loop {
            let vec;
            match self.input_channel.recv() {
                Ok(val) => vec = val,
                Err(_e) => break,
            }

            let th_message = ThreadMessage::new_with_stateless(self.handler.clone(), vec);
            s.send(th_message).unwrap();
        });
    }
}
