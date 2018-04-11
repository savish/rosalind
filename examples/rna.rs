extern crate dna;

use dna::*;
use std::io;

fn main() {
    let mut dna_string = String::new();
    io::stdin().read_line(&mut dna_string).unwrap();
    println!("{}", RNA::from(DNA::new(&dna_string)));
}
