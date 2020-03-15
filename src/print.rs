pub fn run() {
    // Print to console
    println!("Hello from the print.rs file");

    // Basic Formating
    println!("Number: {}", 1);
    println!("{} is from {}. He is {} yrs old", "Eric", "Michigan", 38);

    // Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "Eric", "Michigan", "code");

    // Named Arguments
    println!("{name} likes {activity}.", name = "Eric", activity = "coding");

    // Placeholder Traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "Hello"));

    // Basic Math
    println!("10 + 10 = {}", 10 + 10);

    
}