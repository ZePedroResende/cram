use std::boxed::Box;
use std::collections::HashMap;
use std::thread;
use std::vec::Vec;
use crossbeam::crossbeam_channel::{unbounded,TryRecvError};
use crossbeam::{Receiver,Sender};
use crate::serializers::*;
use crate::node::thread_pool::ThreadMessage;

use super::Label;

impl Label {
    pub fn new<F>( default_fun: F, input_channel : Receiver<Vec<u8>> ) -> Label 
    where F: FnMut(Vec<u8>) + Send + Sync + 'static,{  
        
        let (s,r) = unbounded();

        Label {          
            input_channel: input_channel,  
            default_fun : Some( Box::new( default_fun)),
            map : HashMap::new(),
            fn_receiver : r,
            fn_sender : s,
            queue : HashMap::new(),
        }
    }
    

    pub fn start( mut self, s : Sender<ThreadMessage>){

        self.queue.insert(None, Vec::new());

        for k in self.map.keys(){
            self.queue.insert( Some(k.clone()), Vec::new() );
        }

        thread::spawn(move || loop {

            loop{
                match self.fn_receiver.try_recv(){
                    Ok((option_label, fun)) => {
                        match option_label.clone() {
                            None =>
                                self.default_fun = Some(fun),
                            Some(label) =>{
                                self.map.insert( label, Some(fun));
                            }
                        }
                        // clear buffer
                        self.queue.get_mut(&option_label).unwrap().pop();

                    },
                    Err(TryRecvError::Empty) =>
                        break,
                    Err(TryRecvError::Disconnected) => 
                        panic!("Channel disconnected"),
                }
            }

            let vec =  self.input_channel.recv().unwrap();
            
            let (label,msg) = deserialize_label_message(&vec);
            
            match self.map.remove(&label) {
                None => {
                    match self.default_fun{
                        Some(fun) =>{
                            s.send( ThreadMessage::new_with_stateful(fun, msg, None, self.fn_sender.clone())).unwrap();
                            self.default_fun = None;
                        }
                        None =>{
                            self.queue.get_mut(&None).unwrap().push( vec );
                        }
                    }
                },
                Some(option_fun) =>{
                    match option_fun{
                        Some(fun) =>{
                           self.map.insert( label.clone(), None);  
                            s.send( ThreadMessage::new_with_stateful(fun, msg, Some(label.clone()), self.fn_sender.clone())).unwrap();
                        },
                        None =>{
                            self.queue.get_mut(&Some(label)).unwrap().push( vec );
                        }
                    }
                }
                    
            };
            
        });
    
    }


    pub fn add_handlers(&mut self, list :  Vec<( String, Box<FnMut(Vec<u8>) + Send + Sync + 'static >)>){

        for (l,f) in list {
            self.map.insert(l, Some(f));
        }
    }


    pub fn add_handler(&mut self,  label : String, fun : Box<FnMut(Vec<u8>) + Send + Sync + 'static>) {
        self.map.insert( label, Some(fun) );
    }


}