// Varibles hold primitive data or references to data
// Variables are immuntable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Eric";

    // Without mut (mutable) you cannot reassign the variable.
    let mut age = 37;

    age = 38;

    println!("My name is {} and I am {}", name, age);
}