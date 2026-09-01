mod member;

use std::fs::*;

fn main() {
    // let args = std::env::args();
    // if args.len() != 2 {
    //     eprintln!("Usage: gzip <file>");
    // }

    let input_file = std::env::args().nth(1).expect("Usage: gzip <file>");
    if !exists(&input_file).unwrap_or(false) {
        eprintln!("File does not exist!");
        return;
    }

    let file = File::open(&input_file).expect("Unable to open file");
}
