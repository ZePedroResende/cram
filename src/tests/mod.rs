use crate::serializers::*;
use crate::node::*;
use crossbeam::crossbeam_channel::unbounded;

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
fn test_controller(){
    
    let (s1,r1) = unbounded();
    let (s2,r2) = unbounded();
    let (s3,r3) = unbounded();

    let h1 = move |v:Vec<u8>| { s1.send(v.clone()).unwrap(); };
    let h2 = move |v:Vec<u8>| { s2.send(v.clone()).unwrap(); };
    let h3 = move |v:Vec<u8>| { s3.send(v.clone()).unwrap(); };

    let node =  Builder::new(11101)
                    .set_simple_controller(h1)
                    .set_label_controller(h2)
                    .add_label_handler("label".to_string(), h3)
                    .build(3);
        
    let vec_1 = "my first message".as_bytes().to_vec();
    let vec_2 = "another message".as_bytes().to_vec();
    let vec_3 = "random message".as_bytes().to_vec();
    let vec_4 = "last message".as_bytes().to_vec();


    node.send( vec_1.clone(),  "localhost:11101".to_string());     
    
    node.send( vec_2.clone(), "localhost:11101".to_string()); 
    
    node.send_with_label( vec_3.clone(), "label".to_string(), "localhost:11101".to_string());
    
    node.send_with_label( vec_4.clone(), "randomLabel".to_string(), "localhost:11101".to_string());


    assert_eq!( r1.recv(), Ok(vec_1) );  
    assert_eq!( r1.recv(), Ok(vec_2) );  
    assert_eq!( r3.recv(), Ok(vec_3) );  
    assert_eq!( r2.recv(), Ok(vec_4) );  
}
