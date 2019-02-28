use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::collections::HashMap;


pub struct Ram{
    ctx : zmq::Context,
    push_sockets : Arc<Mutex<Vec<zmq::Socket>>>,
    pull_socket : Arc<Mutex<zmq::Socket>>,
    handlers : Arc<Mutex< HashMap<String,fn(String)> >>,
    default_handler : Arc<Mutex<fn(String)>>,
}


impl Ram{
    
    /*      Constructor     */

    pub fn new( port : i32) -> Ram {


        let context = zmq::Context::new();
        let pull_socket = context.socket(zmq::PULL).unwrap();
        pull_socket.bind( &format!("tcp://*:{}", port)).unwrap();

        Ram{
            ctx : context,
            push_sockets : Arc::new(Mutex::new(Vec::new())),
            pull_socket : Arc::new(Mutex::new(pull_socket)),
            handlers : Arc::new(Mutex::new(HashMap::new())),
            default_handler : Arc::new( Mutex::new( | _ : String| {} )),
        }
    }
 

    /*      Methods     */ 

    pub fn start(&self) -> JoinHandle<()>{

        let pull_socket2 = self.pull_socket.clone();
        let handlers2 = self.handlers.clone();
        let default_handler2 = self.default_handler.clone();

        let handler = thread::spawn( move ||{

            loop {
                let msg = pull_socket2.lock().unwrap().recv_msg(0).unwrap();
                
                let parts : Vec<&str> = msg.as_str().unwrap().split("||").collect();
                
                let tag = parts[0].to_string();
                let msg = parts[1].to_string();

                

                match handlers2.lock().unwrap().get( &tag ) {
                    Some(func) => func(msg),
                    None => default_handler2.lock().unwrap()(msg),
                }

            };
        });

        handler
    }

    /*  Add output connection to send messages */
    pub fn add_connection(&self, address : String){
        let socket = self.ctx.socket(zmq::PUSH).unwrap();
        socket.connect( &format!("tcp://{}", address) ).unwrap();
        let mut push_sockets = self.push_sockets.lock().unwrap();
        push_sockets.push(socket);
    }
    
    /* Add function to apply when receive messages with that tag */ 
    pub fn register_handler(&self, tag : String, fun : fn(String)){
        self.handlers.lock().unwrap().insert( tag, fun);
    }


    /* Add function to apply when receive messages with that not registered tag  */ 
    pub fn register_default_handler(&self, fun : fn(String) ){
        let mut df = self.default_handler.lock().unwrap();
        *df = fun;

    }

    /*  Send message @msg with tag @tag to all output connections */ 
    pub fn broadcast(&self, tag : String, msg : String){
        
        let list = self.push_sockets.lock().unwrap();

        let new_message = format!("{}||{}", tag, msg);
        
        //println!("[server] send:{}", new_message);
        
        let bytes = new_message.as_bytes();

        for socket in list.iter(){
            socket.send(&bytes,0).unwrap();
        }
    }
    
}
