use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let argv: Vec<String> = env::args().collect();
    for (argc, argv) in argv.iter().skip(1).enumerate() {
        println!("argv[{}] = {}", argc, argv);
        let mut file = File::open(argv).unwrap();
        let mut src = String::new();
        file.read_to_string(&mut src).unwrap();
        println!("{}", src);
    }
}
