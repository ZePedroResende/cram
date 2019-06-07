use std::collections::HashMap;
use std::fs;

pub struct Dns{
    id_to_address : HashMap<String, String>,
    _address_to_id : HashMap<String, String>,
} 

impl Dns{
    pub fn new( dns_path : String) -> Dns{
       
        let mut address_to_id = HashMap::new();
        let mut id_to_address = HashMap::new();

        // load file 

        let contents = fs::read_to_string(dns_path)
            .expect("Something went wrong reading the file");
        
        let mut lines= contents.lines();

        loop {
            match lines.next(){
                Some(line) =>{
                    let vec = line.split(" ").collect::<Vec<&str>>();
                    if vec.len() == 2 {
                        address_to_id.insert(vec[1].to_string(), vec[0].to_string());
                        id_to_address.insert(vec[0].to_string(), vec[1].to_string());
                    }
                },
                None => 
                    break,
            }    
        }   

        Dns{
            _address_to_id : address_to_id,
            id_to_address : id_to_address,
        }
    }


    pub fn get_address(&self, id : &String) -> Option<String>{
       self.id_to_address.get(id).cloned()
    }

}
