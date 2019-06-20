use std::boxed::Box;
use std::collections::HashMap;
use std::thread;
use std::vec::Vec;
use crossbeam::crossbeam_channel::{unbounded, Select};
use crossbeam::{Receiver,Sender};
use queue::Queue;
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
            
            queue_by_label : HashMap::new(),
            pending : 0,
            stoped : false,
        }
    }
    

    pub fn start( mut self, thread_pool_channel : Sender<ThreadMessage>){
        
        // initialize queue
        self.queue_by_label.insert(None, Queue::new());

        for k in self.map.keys(){
            self.queue_by_label.insert( Some(k.clone()), Queue::new());
        }
        
        let fn_receiver_cloned = self.fn_receiver.clone();
        let input_channel_cloned = self.input_channel.clone();

        thread::spawn(move ||  {
            
            let mut sel = Select::new();        
            sel.recv( &fn_receiver_cloned );
            sel.recv( &input_channel_cloned);

            loop{
                match sel.ready() {
                    0 => {
                        // From threadPool
                        let res = self.fn_receiver.try_recv();
                        if let Err(e) = res {
                            if e.is_empty() { continue; }
                        }
                        let (option_label, f) = res.unwrap();
                        self.receive_handler(thread_pool_channel.clone(), option_label, f);
                        if self.stoped && self.pending == 0{
                            return ;
                        }
                    },
                    _ => {
                        // From outside 
                        let res = self.input_channel.try_recv();
                        if let Err(e) = res {
                            if e.is_empty() { 
                                continue; 
                            }else{
                                self.stoped = true;
                                if self.pending == 0{
                                    return;
                                } 
                                    
                            }
                        };
    
                        let vec = res.unwrap();  
                        
                        let (label,msg) = deserialize_label_message(vec);

                        match self.map.remove(&label) {
                            None => {
                                match self.default_fun{
                                    Some(fun) =>{
                                        thread_pool_channel.send( ThreadMessage::new_with_stateful( fun, msg, None, self.fn_sender.clone())).unwrap();
                                        self.default_fun = None;
                                    }
                                    None =>{
                                        self.queue_by_label.get_mut(&None).unwrap().queue(msg).unwrap();
                                        self.pending += 1;
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
                                        self.queue_by_label.get_mut(&Some(label)).unwrap().queue( msg ).unwrap();
                                        self.pending += 1;                        
                                    }
                                }
                            },
                        };
                    },
                }            
            }
        });
    }

    fn receive_handler(&mut self , thread_pool_channel : Sender<ThreadMessage>, option_label : Option<String>, fun : Box<FnMut(Vec<u8>) + Send + Sync + 'static>){

        let optional_vec = self.queue_by_label.get_mut(&option_label).unwrap().dequeue();

        match option_label {
            None =>
                match optional_vec {
                    None => 
                        self.default_fun = Some(fun),
                    Some(vec) =>{
                        thread_pool_channel.send( ThreadMessage::new_with_stateful(fun, vec, None, self.fn_sender.clone())).unwrap();
                        self.pending -= 1;
                    }
                },
            Some(label) =>{
                match optional_vec{
                    None =>{
                        self.map.insert( label, Some(fun));
                    },
                    Some(vec) =>{
                        thread_pool_channel.send( ThreadMessage::new_with_stateful(fun, vec, Some(label), self.fn_sender.clone())).unwrap();
                        self.pending -= 1;
                    }
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