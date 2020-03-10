use std::env;
use std::process;

#[derive(Debug)]
enum Flavor {
    Vanilla,
    Chocolate
}

#[derive(Debug)]
struct Config {
    filename: String,
    interactive: bool,
}

fn parse_config() -> Config {
    let args: Vec<String> = env::args().collect();
    let interactive: bool = env::args().any(|x| x == "-i");
    println!("Args {:?}", args);
    // zero is the program name itself
    let filename : &String = &args[1];
    Config { filename: filename.clone(), interactive }
}

fn main() {
    println!("Hello, world!");
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("Guess {}", guess);
    println!("Flavor {:?}", Flavor::Vanilla);
    let config: Config = parse_config();
    println!("Config {:?}", config);
    // eprintln!("This goes to stderr");
    // process::exit(1);
}
