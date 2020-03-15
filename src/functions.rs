pub fn run() {
    greeting("Hello", "Eric");

    // Bind function values to variables
    //2147483648 max
    let get_sum = add(2147483647, 5);
    greeting("Hello", "Eric");
    println!("Sum: {}", get_sum);

    // Closure
    let add_nums = |n1: i32 , n2: i32| n1 + n2;
    println!("C Sum: {}", add_nums(3, 3));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}

