use crossbeam::Sender;
use std::sync::Arc;
use super::{ Stateful, Stateless,ThreadMessage};


impl ThreadMessage{
    pub fn new_with_stateless(  fun : Arc<Box<Fn(Vec<u8>) + Send +  Sync + 'static>>, 
                                message : Vec<u8> 
                              )-> ThreadMessage {
        ThreadMessage{
            is_stateless : true,
            stateful_info : None,
            statefless_info : Some( Stateless{
                fun : fun,
                message: message,
            }),
        }
    }

    pub fn new_with_stateful( fun : Box<FnMut(Vec<u8>) + Send + Sync + 'static >,
                              message : Vec<u8>, label : Option<String>,
                              s : Sender<(Option<String>, Box<FnMut(Vec<u8>) + Send +  Sync + 'static>)>) -> ThreadMessage{
        ThreadMessage{
            is_stateless : false,
            statefless_info : None,
            stateful_info : Some(Stateful{
                fun : fun,
                message : message,
                sender : s,
                label : label,
            }),
        }
    }

    pub fn is_stateless(&self) -> bool{
        self.is_stateless
    }
    
    pub (super) fn get_stateful(self) -> Option<Stateful>{
        self.stateful_info
    }

    pub (super) fn get_stateless(self) -> Option<Stateless>{
        self.statefless_info
    }

}