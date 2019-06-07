#![cfg(test)]

use crate::serializers::*;
use crate::node::{Builder};
use crossbeam::crossbeam_channel::{ RecvError, Select,unbounded};
use std::{thread, time};
use std::str;

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

#[test]
fn test_mut_controller(){
    
    let mut id = 0;
    
    let (s,r) = unbounded();
    
    let ns = s.clone();
    
    let h = move |v:Vec<u8>| { 
        id += 1;
        let mut message = id.to_string().as_bytes().to_vec();
    
        let mut clone = v.clone();
        message.append( &mut " - ".as_bytes().to_vec());
        message.append(  &mut clone );
    
        ns.send( message ).unwrap();
    };  


    let node = Builder::new(3031)
            .set_simple_controller_mut(h)
            .build(1);



    let message = "ola mundo".as_bytes().to_vec();

    for _i in 1..5{
        node.send( message.clone(), "me".to_string() );
    }

    thread::sleep(time::Duration::from_millis(100));

    loop{
        match r.try_recv(){
            Ok(v) => {
                println!(" {:?}",  str::from_utf8(&v).unwrap() );            
            },
            Err(_e) => 
                break,
        }
    }    

}


#[test]
fn new_test(){

    let node_builder = Builder::new(11103);
    let simple;
    let upper;
    let duplicate;
    let default;

    {
        let node_config = node_builder.get_shallow_node();
        let mut count = 0;
        simple = move |v : Vec<u8>|{
            count += 1; 
            println!("receive message {} : {:?}", count, str::from_utf8(&v).unwrap());
            node_config.send_with_label( v, "upper".to_string(), "localhost:11103".to_string() ); 
        };
    }
    {
        let node_config = node_builder.get_shallow_node();
        upper = move |v : Vec<u8>|{
            let message = String::from_utf8(v).unwrap().to_uppercase();
            node_config.send_with_label( message.as_bytes().to_vec(), "duplicate".to_string(), "localhost:11103".to_string() ); 
        };
    }
    {
        let node_config = node_builder.get_shallow_node();
        duplicate = move |v : Vec<u8>|{
            let message = String::from_utf8(v).unwrap().repeat(2);
            node_config.send_with_label( message.as_bytes().to_vec(), "none".to_string(), "localhost:11103".to_string() ); 
        };
    }

    default = move | v : Vec<u8>|{
        println!("Complete : {:?}", str::from_utf8(&v).unwrap());
    };   

    let final_confi = node_builder.set_simple_controller_mut(simple)
                .set_label_controller(default)
                .add_label_handler("duplicate".to_string(),duplicate )
                .add_label_handler("upper".to_string(), upper)
                .build(2);
    
    
    final_confi.send( "ola mundo".as_bytes().to_vec(), "localhost:11103".to_string());
    final_confi.send( "Eu sou o quim".as_bytes().to_vec(), "localhost:11103".to_string());
    final_confi.send( "isto devia dar".as_bytes().to_vec(), "localhost:11103".to_string());
    
    thread::sleep(time::Duration::from_millis(100));
}

#[test]
fn test_mut_label(){

    let mut x1 = 0;
    let h = move |_v:Vec<u8>|{ x1 += 1; println!("x = {}", x1) };    

    let config = Builder::new(3031)
                        .set_label_controller_mut( |_v : Vec<u8>| {println!("Erro");})
                        .add_label_handler_mut("label2".to_string(), h)
                        .build(1);
    
    config.send_with_label( "ola".as_bytes().to_vec(), "label2".to_string(), "localhost:3031".to_string());
    config.send_with_label( "ola".as_bytes().to_vec(), "label2".to_string(), "localhost:3031".to_string());
    config.send_with_label( "ola".as_bytes().to_vec(), "label2".to_string(), "localhost:3031".to_string());
    config.send_with_label( "ola".as_bytes().to_vec(), "label2".to_string(), "localhost:3031".to_string());
    config.send_with_label( "ola".as_bytes().to_vec(), "label2".to_string(), "localhost:3031".to_string());

    thread::sleep(time::Duration::from_millis(2000));
}

#[test]
fn test_crossbeam_select(){
    let (s1,r1) = unbounded();
    let (s2,r2) = unbounded();
    

    thread::spawn(move || { 

        // Build a list of operations.
        let mut sel = Select::new();
        
        sel.recv(&r1);
        sel.recv(&r2);
        

        loop{
            let index = sel.ready();
            match index {
                0 =>{
                    let res = r1.try_recv();
                    if let Err(e) = res {
                        if e.is_empty() {
                            continue;
                        }
                    }
                    println!("{}", res.map_err(|_| RecvError).unwrap());
                },
                1 => {
                    let res = r2.try_recv();
                    if let Err(e) = res {
                        if e.is_empty() {
                            continue;
                        }
                    }
                    println!("{}", res.map_err(|_| RecvError).unwrap());
                },
                _ => {
                    println!("Error");
                },
            }
        }
    });
    

    s1.send("ola").unwrap();
    thread::sleep(time::Duration::from_millis(1000));
    s2.send(1).unwrap();
    s1.send("lindo").unwrap();
    s1.send("lindo").unwrap();
    s1.send("lindo").unwrap();
    s1.send("lindo").unwrap();

    thread::sleep(time::Duration::from_millis(1000));
}
