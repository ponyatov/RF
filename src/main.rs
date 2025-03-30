use std::env;

fn main() {
    let argv: Vec<String> = env::args().collect();
    for (argc, argv) in argv.iter().skip(1).enumerate() {
        println!("argv[{}] = {}", argc, argv);
    }
}
