use std::sync::{Arc, Mutex};
use std::collections::HashMap;



pub struct Writer{
    ctx : Arc<Mutex<zmq::Context>>,
    push_sockets : Arc<Mutex<HashMap<String,zmq::Socket>>>,
}

impl Writer{

    pub fn new( context : Arc<Mutex<zmq::Context>> ) -> Writer{
        
        Writer {
            ctx : context,
            push_sockets : Arc::new( Mutex::new(HashMap::new()) ),
        }
    }

    pub fn send(&self, address : String, msg : Vec<u8>){

        let mut push_sockets_lock = self.push_sockets.lock().unwrap();

        if ! push_sockets_lock.contains_key(&address) {
            let sck = self.ctx.lock().unwrap().socket(zmq::PUSH).unwrap();
            sck.connect( &format!("tcp://{}", address) ).unwrap();
            push_sockets_lock.insert( address.clone(), sck);
        }

        let sck = push_sockets_lock.get(&address).unwrap();
  
        sck.send(&msg,0).unwrap();
    }

    pub fn close_all(&self){
        self.push_sockets.lock().unwrap().clear();
    }
}