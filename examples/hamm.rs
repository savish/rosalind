extern crate gen_str;

use gen_str::{hamming_distance, GeneticString, DNA};
use std::io;

fn main() {
    let mut dna1_string = String::new();
    io::stdin().read_line(&mut dna1_string).unwrap();
    let mut dna2_string = String::new();
    io::stdin().read_line(&mut dna2_string).unwrap();

    let dna1 = DNA::new(&dna1_string);
    let dna2 = DNA::new(&dna2_string);
    println!("{}", hamming_distance(dna1.content(), dna2.content()));
}
