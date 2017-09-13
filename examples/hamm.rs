extern crate dna;

use dna::DNA;
use std::io;

fn main() {
    let mut dna1_string = String::new();
    io::stdin().read_line(&mut dna1_string).unwrap();
    let mut dna2_string = String::new();
    io::stdin().read_line(&mut dna2_string).unwrap();

    let dna1 = DNA::new(dna1_string.trim());
    let dna2 = DNA::new(dna2_string.trim());
    println!("{}", dna1.hamm(&dna2));
}