pub mod controller;
pub mod io;

pub mod configuration;
pub mod thread_pool;

use crate::serializers;
use configuration::Configuration;
use controller::stateless;
use controller::stateful;

use io::Io;
use thread_pool::ThreadPool;

use crossbeam::crossbeam_channel::unbounded;

pub struct Builder{
    io : Io,
    
    simple_controller : Option<stateless::Simple>,
    
    simple_controller_mut : Option<stateful::Simple>,
    
    label_controller : Option<stateless::Label>,

    label_controller_mut : Option<stateful::Label>,
    
    list : Vec< (String, Box<Fn(Vec<u8>) + Send + Sync + 'static>)>,
    
    list_mut : Vec< (String, Box<FnMut(Vec<u8>) + Send + Sync + 'static>)>,


    configuration : Configuration,
}

impl Builder{

    pub fn new( port : usize)-> Builder {
        let (s,r) = unbounded();

        let io = Io::new( port, r);
        let config = Configuration::new(s);

        Builder {
            io : io,
            simple_controller : None,
            simple_controller_mut : None,
            label_controller : None,
            label_controller_mut : None,
            list : Vec::new(),
            list_mut : Vec::new(),
            configuration : config,
        }
    }

    pub fn set_simple_controller<F>(mut self,  func :  F ) -> Builder
    where F  : Fn(Vec<u8>) + Send + Sync + 'static{
     
        let (s,r) = unbounded();

        self.simple_controller = Some( stateless::Simple::new(func, r));
        self.simple_controller_mut = None;

        self.configuration.controllers.insert(0,s);
        self
    }

    pub fn set_simple_controller_mut<F>(mut self,  func :  F ) -> Builder
    where F  : FnMut(Vec<u8>) + Send + Sync + 'static{
     
        let (s,r) = unbounded();

        self.simple_controller_mut = Some( stateful::Simple::new(func, r));
        self.simple_controller = None;

        self.configuration.controllers.insert(0,s);
        self
    }

    pub fn set_label_controller<F>(mut self,  default_fun :  F ) -> Builder
    where F  : Fn(Vec<u8>) + Send + Sync + 'static{
     
        let (s,r) = unbounded();

        self.label_controller = Some( stateless::Label::new(default_fun, r) );
        self.label_controller_mut = None;
     
        self.configuration.controllers.insert(1,s);
        self
    }
    
    pub fn set_label_controller_mut<F>(mut self,  default_fun :  F ) -> Builder
    where F  : FnMut(Vec<u8>) + Send + Sync + 'static{
     
        let (s,r) = unbounded();

        self.label_controller_mut = Some( stateful::Label::new(default_fun, r) );
        self.label_controller = None;

        self.configuration.controllers.insert(1,s);
        self
    }
    
    
    pub fn add_label_handler<F>(mut self, label : String, fun : F ) -> Builder
    where F  : Fn(Vec<u8>) + Send + Sync + 'static{
        self.list.push( (label, Box::new(fun)) );
        self
    }

    pub fn add_label_handler_mut<F>(mut self, label : String, fun : F ) -> Builder
    where F  : FnMut(Vec<u8>) + Send + Sync + 'static{
        self.list_mut.push( (label, Box::new(fun)) );
        self
    }
    
    pub fn build(self, num_threads : usize) -> Node {
        
        let pool = ThreadPool::new(num_threads);
      
        match self.label_controller {
            None => (),
            Some(mut lc) => {
                lc.add_handlers(self.list);
                lc.start( pool.get_sender() );
            },
        };

        match self.label_controller_mut {
            None => (),
            Some(mut lc) => {
                lc.add_handlers(self.list_mut);
                lc.start( pool.get_sender() );
            },
        };

        match self.simple_controller {
            None => (),
            Some(sc) => sc.start( pool.get_sender() ),
        }
        
        match self.simple_controller_mut {
            None => (),
            Some(sc) => sc.start( pool.get_sender() ),
        }
        
        self.io.start(self.configuration.clone());
        
        Node{
            config : self.configuration.clone(),
            pool : pool,
        }
    } 
}

pub struct Node{
    config : Configuration,
    pool : ThreadPool,
}


impl Node{
    pub fn send(&self, msg : Vec<u8>, address : String){
        self.config.io.send( (0,msg,address)).unwrap();
    }   

    pub fn send_with_label(&self, msg : Vec<u8>, label : String, address : String){
        let new_msg = serializers::serialize_label_message(&label, &msg);
        self.config.io.send( (1,new_msg,address)).unwrap();
    }

    pub fn join(&self){
        self.pool.join()
    }
}