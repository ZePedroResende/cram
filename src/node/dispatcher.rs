use crossbeam::crossbeam_channel::unbounded;
use crossbeam::Receiver;
use crossbeam::Sender;
use std::thread;
use std::collections::HashMap;

use crate::serializers::*;


pub fn start_dispatcher( map : HashMap<i8, Sender<(Vec<u8>, String)>>, input_channel : Receiver<(Vec<u8>, String)> ){
        
    thread::spawn(move || loop {
        let ( mut vec, from) : (Vec<u8>,_) = input_channel.recv().unwrap();
        
        let message_type = get_type(&mut vec);

        match map.get(&(message_type as i8)){
            None => (),
            Some(c) => c.send( (vec, from) ).unwrap(),
        };
    });

}


