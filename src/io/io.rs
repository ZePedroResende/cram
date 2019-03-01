mod io  


pub struct Io{
    ctx : zmq::Context,
    reader : io::Reader,    
    writer : io::Writer,
}


impl Io{
    
    pub fn new( port : i32) -> Io {

        let context = zmq::Context::new();
         
        Io{
            ctx : context,
            reader : io::Reader::new(context, port),
            writer : io::Write::new(context),
        }
    }

    pub read(self) -> (String, [u8]){
        self.read.read()
    }


    pub write(self, label : String, msg : String){
        self.write.broadcast(label, msg);
    }
    
}
