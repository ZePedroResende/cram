mod protos;

use protos::LabelMessage;

use std::borrow::Cow;
use quick_protobuf::{deserialize_from_slice, serialize_into_vec};



pub fn deserialize_label_message( bytes : &Vec<u8> ) -> ( String, Vec<u8>){
    let message : LabelMessage = deserialize_from_slice(&bytes).expect("Cannot write message!");
    (message.label.to_string(), message.msg.iter().cloned().collect(), )
}

pub fn serialize_label_message( label : &String, msg : &Vec<u8>) -> Vec<u8>{

    let new_message = LabelMessage{
            label :  Cow::Borrowed(&label),
            msg :  Cow::Borrowed(&msg),
        };

    serialize_into_vec( &new_message).expect("Cannot write message!")    
}

pub fn put_type( message_type : i8,  bytes : &mut Vec<u8>){
     bytes.insert(0, message_type as u8)
}

pub fn get_type( bytes : &mut Vec<u8>) -> i8 {
    bytes.remove(0) as i8
}

