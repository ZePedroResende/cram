use std::{thread, time};
use std::sync::{Arc, Mutex};

mod ram;
mod bot;


fn main() {

   test1();
}


fn test1(){

    let my_ram = ram::Ram::new(1101);
    
    let context =  Arc::new(Mutex::new(zmq::Context::new()));

    my_ram.add_connection( String::from("localhost:1102") );

    my_ram.register_handler( String::from("msg"), |msg| println!("[Handler] receive:{}", msg) );

    my_ram.register_default_handler( |msg| println!("[Default handler] receive:{}", msg)) ;

    let bot = bot::pull_client(1102, context.clone() );

    let bot2 = bot::push_client( String::from("localhost:1101"), context.clone(), String::from("msg"));

    my_ram.start();
    
    for _ in 0..10 {
        my_ram.broadcast(String::from("msg"), String::from("mundo") );
        
        thread::sleep(time::Duration::from_millis(500));   
    }
    
    bot2.join().unwrap();
    bot.join().unwrap();
}




fn test2(){
    
    let my_ram1 = ram::Ram::new(1101);
    let my_ram2 = ram::Ram::new(1102);

    my_ram1.add_connection( String::from("localhost:1101"));

    my_ram2.add_connection( String::from("localhost:1102"));

    let handler_default = | msg : String| println!("End - {}", msg);
    
    my_ram1.register_default_handler( handler_default );

    my_ram2.register_default_handler( handler_default );
    
    let closure = |msg : String| {

        let mut value = msg.parse::<i32>().unwrap();

        if value < 10 {    
        
            value += 1; 

            my_ram1.broadcast( String::from("msg"), value.to_string());        
        }
        else{
            //self.broadcast(String::from("reply"),  String::from("fim do ciclo"));
        }
    };

    my_ram1.register_handler( String::from("msg"), closure );

}