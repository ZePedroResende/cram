
pub mod SimpleController;
pub mod LabelController;
 

pub trait Controller{
    
    fn get_message(&self) ->  ( Vec<u8>, String);
        
    fn call(&self, message : &Vec<u8>) -> ();
} 
