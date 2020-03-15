/*
Primitive Types:--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (the 
number of bits they take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/
/* Rust is a satically typed language, which means that it must know the  types
of all variables at compile time, however, the compiler  can usually infer what 
type we want to use based on the value and how we use it. 
*/
pub fn run() {
    println!("Hello from types::run()");

    // Default is "i32"
    let x = 1;

    // Float Default is f64 
    let y = 2.5;

    // Add explicit type
    let z :i64 = 4545454545454;
    
    println!("Max is i32 {}", std::i32::MAX);
    println!("Max is i64 {}", std::i64::MAX);
    println!("{} default i32, {} default f64, explicit :i64 {}", x, y, z);

    // Boolean
    let is_active: bool = true;
    println!("{:?}", (x, y, z, is_active));
}