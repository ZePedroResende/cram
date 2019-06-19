use super::Builder;
use super::Node;
use super::io::Io;
use super::controller::{stateful,stateless};
use super::thread_pool::ThreadPool;
use crossbeam::crossbeam_channel::{unbounded};

impl Builder{
    pub fn new( port : usize)-> Builder {
        let (s,r) = unbounded();

        let io = Io::new( port, r);
        let config = Node::new(port, s);

        Builder {
            io : io,
            simple_controller : None,
            simple_controller_mut : None,
            label_controller : None,
            label_controller_mut : None,
            list : Vec::new(),
            list_mut : Vec::new(),
            configuration : config,
            has_controllers : false,
        }
    }

    pub fn new_with_dns( port : usize, dns_path : String)-> Builder {
        let (s,r) = unbounded();

        let io = Io::new_with_dns(port, r, dns_path);
        let config = Node::new(port, s);

        Builder {
            io : io,
            simple_controller : None,
            simple_controller_mut : None,
            label_controller : None,
            label_controller_mut : None,
            list : Vec::new(),
            list_mut : Vec::new(),
            configuration : config,
            has_controllers : false,
        }
    }
    pub fn get_shallow_node(&self) -> Node{
        self.configuration.clone()
    }

    pub fn set_simple_controller<F>(mut self,  func :  F ) -> Builder
    where F  : Fn(Vec<u8>) + Send + Sync + 'static{
        
        let (s,r) = unbounded();

        self.simple_controller = Some( stateless::Simple::new(func, r));
        self.simple_controller_mut = None;

        self.configuration.controllers.insert(0,s);
        self.has_controllers = true;
        self
    }

    pub fn set_simple_controller_mut<F>(mut self,  func :  F ) -> Builder
    where F  : FnMut(Vec<u8>) + Send + Sync + 'static{
     
        let (s,r) = unbounded();

        self.simple_controller_mut = Some( stateful::Simple::new(func, r));
        self.simple_controller = None;

        self.configuration.controllers.insert(0,s);
        self.has_controllers = true;
        self
    }

    pub fn set_label_controller<F>(mut self,  default_fun :  F ) -> Builder
    where F  : Fn(Vec<u8>) + Send + Sync + 'static{
     
        let (s,r) = unbounded();

        self.label_controller = Some( stateless::Label::new(default_fun, r) );
        self.label_controller_mut = None;
     
        self.configuration.controllers.insert(1,s);
        self.has_controllers = true;
        self
    }
    
    pub fn set_label_controller_mut<F>(mut self,  default_fun :  F ) -> Builder
    where F  : FnMut(Vec<u8>) + Send + Sync + 'static{
     
        let (s,r) = unbounded();

        self.label_controller_mut = Some( stateful::Label::new(default_fun, r) );
        self.label_controller = None;

        self.configuration.controllers.insert(1,s);
        self.has_controllers = true;
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
        
        if ! self.has_controllers {
            self.io.start(self.configuration.clone());
            return self.configuration;
        }
        
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
            Some(sc) => sc.start(),
        }
        
        self.io.start(self.configuration.clone());
        
        
        self.configuration
    } 

}
