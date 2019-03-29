mod reader;
mod writer;

use reader::Reader;
use writer::Writer;

use crate::serializers::*;
use std::sync::{Arc, Mutex};

pub struct Io {
    reader: Reader,
    writer: Writer,
}

impl Io {
    pub fn new(port: i32) -> Io {
        let context = Arc::new(Mutex::new(zmq::Context::new()));

        Io {
            reader: Reader::new(context.clone(), port),
            writer: Writer::new(context.clone()),
        }
    }

    pub fn read(&self) -> (Vec<u8>, i8) {
        get_type(&self.reader.read())
    }

    pub fn send(&self, address: String, msg: &Vec<u8>, msg_type: i8) {
        let mut msg_clone = msg.to_vec();
        put_type(msg_type, &mut msg_clone);
        self.writer.send(address, msg_clone);
    }

    pub fn close_all(&self) {
        self.writer.close_all();
    }
}
