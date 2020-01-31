use std::process;
use ocariner::*;

fn main() {
    let mut oc = OcTable::new();
    oc.generate_notes();
    if let Err(e) = oc.render() {
        eprintln!("Error when displaying the table: {}", e);
        process::exit(1);
    }    
}