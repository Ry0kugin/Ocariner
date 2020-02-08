use std::process;
use std::env;
use ocariner::*;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("ERROR in OcTable structure: {}", e);
        process::exit(1);
    }    
}