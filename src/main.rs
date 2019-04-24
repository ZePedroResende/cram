#![allow(unused_imports)]
#![allow(dead_code)]

mod node;
mod serializers;
mod tests;

use node::Node;

fn main() {
    let mut n = Node::new(5000);
    n.start();
    println!("ola mundo");
}
