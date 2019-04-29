use std::boxed::Box;
use std::collections::HashMap;
use std::ops::Fn;
use std::thread;
use std::vec::Vec;
use crossbeam::crossbeam_channel::unbounded;
use crossbeam::Receiver;
use crossbeam::Sender;

use super::Controller;
use crate::serializers::*;


pub struct LabelController {
    controller_type : i8,
    input_channel: Sender<(Vec<u8>, String)>,
    output_channel: Sender<(Vec<u8>, String)>,
}

impl LabelController {
    pub fn new<F>( default_fun: F, 
                   map : HashMap<String, Box<Fn(&Vec<u8>) + Send + Sync + 'static>>,
                   output_channel : Sender<(Vec<u8>, String)> ) -> LabelController 
     where
        F: Fn(&Vec<u8>) + Send + Sync + 'static,
    {
        let (s, r) = unbounded();

        thread::spawn(move || loop {
            let (vec, _from) = r.recv().unwrap();

            let (label,msg) = deserialize_label_message(&vec);
            
            match map.get(&label){
                None => 
                    (default_fun)(&msg),
                Some(f) =>
                    f(&msg),
            };
        });

        LabelController {
            controller_type : 1,
            input_channel: s,
            output_channel: output_channel,
        }
    }

    pub fn reply(&self, message: Vec<u8>, label: String, address: String) {
        let mut bytes = serialize_label_message(&label, &message);
        put_type( self.controller_type, &mut bytes);
        self.output_channel.send((bytes, address)).unwrap();
    }

    pub fn get_input_channel(&self) -> Sender<(Vec<u8>, String)> {
        self.input_channel.clone()
    }

}

