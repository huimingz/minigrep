use std::env;

macro_rules! print_hello {
    () => {
        println!("Hello, Rust!");
    };
}

fn main() {
    println!("Hello, world!");

    print_hello!();

    let args: Vec<String> = env::args().collect();
    dbg!(args);
}
