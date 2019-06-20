use crossbeam::Receiver;
use std::boxed::Box;
use std::vec::Vec;
use std::thread;

use super::Simple;

impl Simple {
    pub fn new<F>(fun: F, input_channel : Receiver<Vec<u8>> ) -> Simple
    where
        F: FnMut(Vec<u8>) + Send + Sync + 'static,
    {   
        Simple {
            input_channel : input_channel,
            handler : Box::new(fun),
        }
    }

    pub fn start(mut self){
        thread::spawn(move || loop {
            let vec;  
            match self.input_channel.recv() {
                Ok(val) => vec = val,
                Err(_e) => break,
            }
            (self.handler)(vec);    
        });
    }
}
