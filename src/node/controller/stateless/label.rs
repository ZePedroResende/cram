use std::boxed::Box;
use std::collections::HashMap;
use std::ops::Fn;
use std::thread;
use std::vec::Vec;
use crossbeam::{Receiver,Sender};
use std::sync::Arc;

use crate::serializers::*;
use crate::node::thread_pool::ThreadMessage;

use super::Label;

impl Label {
    pub fn new<F>( default_fun: F, input_channel : Receiver<Vec<u8>> ) -> Label 
    where
        F: Fn(Vec<u8>) + Send + Sync + 'static,
    {

        Label {          
            input_channel: input_channel,
          
            default_fun : Arc::new( Box::new( default_fun)),
            map : HashMap::new(),
        }
    }
    

    pub fn start(self, s : Sender<ThreadMessage>){

        thread::spawn(move || loop {

            let vec =  self.input_channel.recv().unwrap();

            let (label,msg) = deserialize_label_message(&vec);
            
            match self.map.get(&label) {
                None => {
                    let tm = ThreadMessage::new_with_stateless( self.default_fun.clone(), msg);
                    s.send(tm).unwrap();        
                }
                Some(f) =>{
                    let tm = ThreadMessage::new_with_stateless( f.clone(), msg);
                    s.send(tm).unwrap();
                }
            };
        });
    }

    pub fn add_handlers(&mut self, list :  Vec<( String, Box<Fn(Vec<u8>) + Send + Sync + 'static >)>){

        for (l,f) in list{
            self.map.insert(l, Arc::new(f));
        }
    }

}

