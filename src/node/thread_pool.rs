use std::sync::Arc;
use crossbeam::crossbeam_channel::unbounded;
use crossbeam::{Receiver,Sender};
use std::boxed::Box;



/// /// info to send to about stateful or stateless /// ///

struct Stateful{
    fun : Box<FnMut(Vec<u8>) + Send +  Sync + 'static>,
    message : Vec<u8>,
    sender : Sender< (Option<String>, Box<FnMut(Vec<u8>) + Send +  Sync + 'static>) >,
    label : Option<String>,
}

struct Stateless{
    fun : Arc<Box<Fn(Vec<u8>) + Send +  Sync + 'static>>,
    message : Vec<u8>,
}


/// /// Message to send to threadPoll /// ///

pub struct ThreadMessage{
    is_stateless : bool,    
    stateful_info : Option<Stateful>,
    statefless_info : Option<Stateless>,
}

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
    
    fn get_stateful(self) -> Option<Stateful>{
        self.stateful_info
    }

    fn get_stateless(self) -> Option<Stateless>{
        self.statefless_info
    }

}

pub struct ThreadPool{
    pool : threadpool::ThreadPool,
    num_threads : usize,
    thread_channel : Receiver<ThreadMessage>,
    input_channel : Sender<ThreadMessage>,
}

impl ThreadPool{
    pub fn new( num_threads : usize) -> ThreadPool{
        
        let (s,r) = unbounded();
        
        let pool = threadpool::ThreadPool::new(num_threads);
        
        for _ in 1..num_threads{
        
            let recv = r.clone();
        
            pool.execute(move || loop{
        
                let th_message : ThreadMessage= recv.recv().unwrap();
             
                if th_message.is_stateless(){
                    let info = th_message.get_stateless().unwrap();
                    (info.fun)(info.message); 
        
                }else{
                    let info = th_message.get_stateful().unwrap();
                    let mut f = info.fun;
                    f(info.message); 
                    info.sender.send((info.label, f)).unwrap();
                }
            });
        }

        ThreadPool{
            pool : pool,
            num_threads : num_threads,
            input_channel : s,
            thread_channel : r,
        } 
    }

    pub fn get_sender(&self) -> Sender<ThreadMessage>{
        self.input_channel.clone()
    }

    pub fn join(&self){
        self.pool.join()
    }

}