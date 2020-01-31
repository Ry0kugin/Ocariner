use std::process;
use ocariner::*;

fn main() {
    let oc = OcTable::new();
    if let Err(e) = oc.render() {
        eprintln!("Error when displaying the table: {}", e);
        process::exit(1);
    }    
}