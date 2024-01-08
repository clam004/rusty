// interface/src/main.rs
mod hello_speaker {
    pub fn greet() {
        println!("hello world");
    }
}

use crate::hello_speaker::greet; // or `use crate::hello_speaker::*;` 

mod sup;
use crate::sup::say_sup; // use crate::sup::*;

fn main() {
    greet();
    say_sup();
}
