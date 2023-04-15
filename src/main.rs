use std::{env, fs, process};
use std::error::Error;

macro_rules! print_hello {
    () => {
        println!("Hello, Rust!");
    };

    ($name:expr) => {
        println!("Hello, {}!", $name);
    };

    ($x:expr, $y:expr) => {
        println!("x: {}, y: {}", $x, $y);
    };
}


fn main() {
    println!("Hello, world!");

    print_hello!();
    print_hello!("yui chan");
    print_hello!("yui", "sakura");

    let args: Vec<String> = env::args().collect();

    // let config = Config::new(&args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    };
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
        // .expect("Should have been able to read the file");

    println!("With text: \n{contents}");

    Ok(())
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
