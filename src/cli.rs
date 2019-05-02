use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    println!("Args: {:?}", args);

    let arg = args[1].clone();
    println!("Arg: {}", arg);
}
