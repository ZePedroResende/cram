mod io;
mod serializers;

use serializers::*;

fn main() {
    
    test_serializers();

    println!("|||||||||\n   FIM\n|||||||||\n");
}

fn test_serializers(){
    
    let label : String = String::from("Ola");
    let msg : Vec<u8> =  b" mundo lindo".to_vec();
    let my_type : i8 = 2;
    
    // Serialize // 
    let mut bytes = serialize_label_message( &label ,&msg);
    putType( my_type, &mut bytes);
    
    
    // desserialize // 

    let (my_type_res, bytes2 ) = getType( &bytes);
    let (label_res, msg_res) = deserialize_label_message( &bytes2 );

    assert_eq!( msg, msg_res);

    assert_eq!( label, label_res);

    assert_eq!( my_type, my_type_res);
}