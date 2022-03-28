use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let rom_file = fs::read(&args[1]).expect("Unable to open ROM file");
    println!("{:?}", args);
}
