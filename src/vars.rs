// Varibles hold primitive data or references to data
// Variables are immuntable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Eric";
    
    // Without mut (mutable) you cannot reassign the variable. 
    let mut age = 37;
    println!("My name is {} and I am {}", name, age);

    age = 38;
    println!("My name is {} and I am {}", name, age);

    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign Multipl vars
    let ( my_name, my_age ) = ("Eric", 38);
    println!("{} is {}", my_name, my_age);
}