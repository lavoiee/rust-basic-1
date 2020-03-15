//mod print;
//mod vars;
//mod types;
//mod loops;
mod functions;
mod pointer_ref;
mod structs;
use std::thread;
use std::time::Duration;

fn main() {
    //vars::run();
    //loops::run();
    //types::run();
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    functions::run();
    pointer_ref::run();
    structs::run();
}
