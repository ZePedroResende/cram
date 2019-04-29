use crate::serializers::put_type;
use crossbeam::crossbeam_channel::unbounded;
use crossbeam::Receiver;
use crossbeam::Sender;

use std::boxed::Box;
use std::ops::Fn;
use std::thread;
use std::vec::Vec;

use super::Controller;

pub struct SimpleController {
    controller_type : i8,
    input_channel: Sender<(Vec<u8>, String)>,
    output_channel: Sender<(Vec<u8>, String)>,
}

impl SimpleController {
    pub fn new<F>(fun: F, output_channel : Sender<(Vec<u8>, String)> ) -> SimpleController
    where
        F: Fn(&Vec<u8>) + Send + Sync + 'static,
    {
        let (s, r) = unbounded();

        thread::spawn(move || loop {
            let (vec, _from) = r.recv().unwrap();
            (fun)(&vec);
        });

        SimpleController {
            controller_type : 0,
            input_channel: s,
            output_channel: output_channel,
        }
    }

    pub fn get_input_channel(&self) -> Sender<(Vec<u8>, String)> {
        self.input_channel.clone()
    }

    pub fn reply(&self, mut message: Vec<u8>, address : String){
        put_type( self.controller_type, &mut message);
        self.output_channel.send( (message,address)).unwrap();
    }
}
