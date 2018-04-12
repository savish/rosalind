extern crate gen_str;

use gen_str::*;
use std::io;

fn main() {
    let mut dna_string = String::new();
    io::stdin().read_line(&mut dna_string).unwrap();
    let dna = DNA::new(&dna_string);
    println!(
        "{} {} {} {}",
        dna.count_symbols()[0],
        dna.count_symbols()[1],
        dna.count_symbols()[2],
        dna.count_symbols()[3]
    );
}
