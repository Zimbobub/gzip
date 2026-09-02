mod flags;
mod header;
mod member;
mod deflate;

use std::{fs::*, io::BufReader};

use crate::member::Member;


pub trait Parse {
    fn read_from_file<R>(buffer: &mut BufReader<R>) -> Option<Self> where R: std::io::Read, Self: Sized;
}


fn main() {
    let input_file = std::env::args().nth(1).expect("Usage: gzip <file>");
    if !exists(&input_file).unwrap_or(false) {
        eprintln!("File does not exist!");
        return;
    }

    let file = File::open(&input_file).expect("Unable to open file");
    let mut reader = BufReader::new(file);
    let member = Member::read_from_file(&mut reader).unwrap();
}
