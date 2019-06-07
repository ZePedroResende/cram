use std::boxed::Box;
use std::collections::HashMap;
use std::thread;
use std::vec::Vec;
use crossbeam::crossbeam_channel::{unbounded, Select};
use crossbeam::{Receiver,Sender};
use crate::serializers::*;
use crate::node::thread_pool::ThreadMessage;

use super::Label;

impl Label {
    pub fn new<F>(  default_fun: F, input_channel : Receiver<Vec<u8>> ) -> Label 
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
    

    pub fn start( mut self, thread_pool_channel : Sender<ThreadMessage>){
        
        // initialize queue
        self.queue.insert(None, Vec::new());

        for k in self.map.keys(){
            self.queue.insert( Some(k.clone()), Vec::new() );
        }
        
        let fn_receiver_cloned = self.fn_receiver.clone();
        let input_channel_cloned = self.input_channel.clone();

        thread::spawn(move ||  {
            
            let mut sel = Select::new();        
            sel.recv( &fn_receiver_cloned );
            sel.recv( &input_channel_cloned);

            loop{
                let index = sel.ready();
                match index {
                    0 => {
                        let res = self.fn_receiver.try_recv();
                        if let Err(e) = res {
                            if e.is_empty() {
                                continue;
                            }
                        }
                        let (option_label, f) = res.unwrap();
                        self.receive_handler(thread_pool_channel.clone(), option_label, f);
                    },
                    1 => {
                        let res = self.input_channel.try_recv();
                        if let Err(e) = res {
                            if e.is_empty() {
                                continue;
                            }
                        }
                        let vec = res.unwrap();
                        let (label,msg) = deserialize_label_message(&vec);

                        match self.map.remove(&label) {
                            None => {
                                match self.default_fun{
                                    Some(fun) =>{
                                        thread_pool_channel.send( ThreadMessage::new_with_stateful( fun, msg, None, self.fn_sender.clone())).unwrap();
                                        self.default_fun = None;
                                    }
                                    None =>{
                                        self.queue.get_mut(&None).unwrap().push(msg);
                                    }
                                }
                            },
                            Some(option_fun) =>{
                                match option_fun {
                                    Some(fun) =>{
                                        self.map.insert( label.clone(), None);  
                                        thread_pool_channel.send( ThreadMessage::new_with_stateful(fun, msg, Some(label.clone()), self.fn_sender.clone())).unwrap();
                                    },
                                    None =>{
                                        self.map.insert( label.clone(), None);  
                                        self.queue.get_mut(&Some(label)).unwrap().push( msg );                        
                                    }
                                }
                            },
                    
                        };
                    },
                    _ => panic!("Erro - index not expected"),
                }            
            }
        });
    }

    fn receive_handler(&mut self , thread_pool_channel : Sender<ThreadMessage>, option_label : Option<String>, fun : Box<FnMut(Vec<u8>) + Send + Sync + 'static>){
        let optional_vec = self.queue.get_mut(&option_label).unwrap().pop();

        match option_label {
            None =>
                match optional_vec {
                    None => 
                        self.default_fun = Some(fun),
                    Some(vec) =>
                        thread_pool_channel.send( ThreadMessage::new_with_stateful(fun, vec, None, self.fn_sender.clone())).unwrap(),
                },
            Some(label) =>{
                match optional_vec{
                    None =>{
                        self.map.insert( label, Some(fun));
                    },
                    Some(vec) =>
                        thread_pool_channel.send( ThreadMessage::new_with_stateful(fun, vec, Some(label), self.fn_sender.clone())).unwrap(),
                }
            }
        }
    }

   
    
    // API //     

    pub fn add_handlers(&mut self, list :  Vec<( String, Box<FnMut(Vec<u8>) + Send + Sync + 'static >)>){

        for (l,f) in list {
            self.map.insert(l, Some(f));
        }
    }

}