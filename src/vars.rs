// Varibles hold primitive data or references to data
// Variables are immuntable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Eric";
    let mut age = 37;

    age = 38;

    println!("My name is {} and I am {}", name, age);
}