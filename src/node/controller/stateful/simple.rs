use crossbeam::crossbeam_channel::unbounded;
use crossbeam::{Receiver, Sender};
use std::boxed::Box;
use std::vec::Vec;
use std::thread;

use crate::node::thread_pool::ThreadMessage;

use super::Simple;

impl Simple {
    pub fn new<F>(fun: F, input_channel : Receiver<Vec<u8>> ) -> Simple
    where
        F: FnMut(Vec<u8>) + Send + Sync + 'static,
    { 
        let (s,r) = unbounded();
        
        Simple {
            input_channel : input_channel,
            handler : Some(Box::new(fun)),
            fn_receiver : r,
            fn_sender : s,
        }
    }

    pub fn start(mut self, s : Sender<ThreadMessage>){
        thread::spawn(move || {
            loop{
                let vec = self.input_channel.recv().unwrap();

                let fun = match self.handler{
                    None => {
                        let (_,f) = self.fn_receiver.recv().unwrap();
                        f
                    },
                    Some(f) => f,
                };

                let th_message = ThreadMessage::new_with_stateful( fun , vec, None, self.fn_sender.clone());

                s.send(th_message).unwrap();

                self.handler = None;
            }    
        });
    }
}
