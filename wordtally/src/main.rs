use std::process;
use wordtally::Config;

fn main() {
    let config = Config::build(std::env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = wordtally::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
