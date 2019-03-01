mod io  

use std::sync::{Arc, Mutex};


struct Writer{
    ctx : zmq::Context,
    push_sockets : Arc<Mutex<Vec<zmq::Socket>>>,
}

impl Writer{

    fn new( context :: zmq::Context ) -> Writer{
        
        Writer{
            ctx : context,
            push_sockets : Arc::new(Mutex::new(Vec::new())),
        }
    }

       /*  Add output connection to send messages */
    fn add_connection(&self, address : String){
        let socket = self.ctx.socket(zmq::PUSH).unwrap();
        socket.connect( &format!("tcp://{}", address) ).unwrap();
        let mut push_sockets = self.push_sockets.lock().unwrap();
        push_sockets.push(socket);
    }

        /*  Send message @msg with tag @tag to all output connections */ 
    fn broadcast(&self, tag : String, msg : String){
        
        let list = self.push_sockets.lock().unwrap();

        let new_message = format!("{}||{}", tag, msg);
        
        //println!("[server] send:{}", new_message);
        
        let bytes = new_message.as_bytes();

        for socket in list.iter(){
            socket.send(&bytes,0).unwrap();
        }
    }
}