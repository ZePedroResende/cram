mod io 

struct Reader{
    pull_socket : Arc<Mutex<zmq::Socket>>,
}

impl Reader{
    
    fn new( context : zmq::Context, port : i32) -> Reader {

        let pull_socket = context.socket(zmq::PULL).unwrap();
        
        pull_socket.bind( &format!("tcp://*:{}", port)).unwrap();

        Reader{
             pull_socket : Arc::new(Mutex::new(pull_socket)),
        }
    }

    fn read(&self) -> (String, [u8]){
    
        let msg = pull_socket2.lock().unwrap().recv_msg(0).unwrap();
                
        let parts : Vec<&str> = msg.as_str().unwrap().split("||").collect();
                
        ( parts[0].to_string(), parts[1].as_bytes() )
        
    }
}