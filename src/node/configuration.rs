use std::collections::HashMap;
use crossbeam::Sender;

pub struct Configuration{
    pub io :  Sender<( i8, Vec<u8>, String) >,
    pub controllers : HashMap< i8, Sender< Vec<u8> > >,
}


impl Configuration{
    pub fn new( io_channel : Sender<(i8, Vec<u8>, String)> ) -> Configuration{
        Configuration{
            io : io_channel,
            controllers : HashMap::new(),
        }
    }
}

impl Clone for Configuration {
    
    fn clone(&self) -> Configuration {
        let mut hash = HashMap::new();
        
        for (k,v) in &self.controllers{
            hash.insert( *k, v.clone() );
        }

        Configuration{
            io: self.io.clone(),
            controllers : hash,
        }
    }
}