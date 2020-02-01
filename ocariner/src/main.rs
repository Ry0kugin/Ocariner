use std::process;
use ocariner::*;

fn main() {
    let mut oc = OcTable::new(0.7);
    if let Err(e) = oc.run() {
        eprintln!("ERROR in OcTable structure: {}", e);
        process::exit(1);
    }    
}