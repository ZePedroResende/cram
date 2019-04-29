
use crate::*;
use crate::node;

use crate::node::io::Io;
use crate::serializers::*;
use crate::node::controller::simple_controller::SimpleController;
use crate::node::*;

use crossbeam::Receiver;
use crossbeam::Sender;
use crossbeam::crossbeam_channel::unbounded;
use std::collections::HashMap;


#[test]
fn test_serializers(){
    
    let label : String = String::from("Ola");
    let msg : Vec<u8> =  b" mundo lindo".to_vec();
    let my_type : i8 = 2;
    
    // Serialize // 
    let mut bytes = serialize_label_message( &label ,&msg);
    put_type( my_type, &mut bytes);
    
    
    // desserialize // 

    let my_type_res = get_type( &mut bytes);
    let (label_res, msg_res) = deserialize_label_message( &bytes );

    assert_eq!( msg, msg_res);

    assert_eq!( label, label_res);

    assert_eq!( my_type, my_type_res);


}


#[test]
fn test_io() {
    
    let port1 = 11101;
    let port2 = 11102;

    let io1 = Io::new(port1);
    let io2 = Io::new(port2);

    let msg = "ola mundo";
    
    io1.get_input_channel().send(( msg.as_bytes().to_vec(), format!("localhost:{}", port2))).unwrap();
    
    let (result,_) = io2.get_output_channel().recv().unwrap();

    assert_eq!( msg.as_bytes().to_vec(), result ); 
}

#[test]
fn test_simple_controller() {
    let (s,r) : (Sender<Vec<u8>>,Receiver<Vec<u8>>)= unbounded();
    
    let s_clone = s.clone();
    
    let func = move |vec : &Vec<u8>| s_clone.send((*vec).clone()).unwrap();

    let io = Io::new(11103);

    let sc = SimpleController::new( func, io.get_input_channel());
    
    let mut map = HashMap::new();
    map.insert(0,sc.get_input_channel());
    
    node::dispatcher::start_dispatcher( map, io.get_output_channel() );

    sc.reply( "ola mundo".as_bytes().to_vec(), "localhost:11103".to_string());

    assert_eq!( "ola mundo".as_bytes().to_vec() , r.recv().unwrap() );
}
