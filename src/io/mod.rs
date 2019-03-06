mod writer;
mod reader;

use writer::Writer;
use reader::Reader;

use std::sync::{Arc, Mutex};


pub struct Io{
    reader : Reader,    
    writer : Writer,
}


impl Io{
    
    pub fn new( port : i32) -> Io {

        let context = Arc::new(Mutex::new(zmq::Context::new()));
         
        Io{
            reader : Reader::new( context.clone(), port),
            writer : Writer::new( context.clone()),
        }
    }

    pub fn read(self) -> Vec<u8> {
        self.reader.read()
    }

    pub fn broadcast(self, msg : Vec<u8>){
        self.writer.broadcast(msg);
    }

    pub fn add_connection(&self, address : String){
        self.writer.add_connection(address);
    }
}
