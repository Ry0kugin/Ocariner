use std::process;
use std::env;
use ocariner::*;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let mut oc = OcTable::new(0.7);
    if let Err(e) = oc.run() {
        eprintln!("ERROR in OcTable structure: {}", e);
        process::exit(1);
    }    
}