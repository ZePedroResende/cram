use std::sync::{Arc, Mutex};

pub struct Writer{
    ctx : Arc<Mutex<zmq::Context>>,
    push_sockets : Arc<Mutex<Vec<zmq::Socket>>>,
}

impl Writer{

    pub fn new( context : Arc<Mutex<zmq::Context>> ) -> Writer{
        
        Writer{
            ctx : context,
            push_sockets : Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add_connection(&self, address : String){

        let socket = self.ctx.lock().unwrap().socket(zmq::PUSH).unwrap();

        socket.connect( &format!("tcp://{}", address) ).unwrap();

        let mut push_sockets = self.push_sockets.lock().unwrap();

        push_sockets.push(socket);
    }

        /*  Send message @msg with tag @tag to all output connections */ 
    pub fn broadcast(&self, msg : Vec<u8>){
        
        let list = self.push_sockets.lock().unwrap();    
    
        for socket in list.iter(){
            socket.send(&msg,0).unwrap();
        }
    }
}