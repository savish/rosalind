extern crate gen_str;

use gen_str::*;
use std::io;

fn main() {
    let mut protein_string = String::new();
    io::stdin().read_line(&mut protein_string).unwrap();

    let protein = Protein::new(&protein_string);
    println!("{}", protein.rna_count(1_000_000).remainder());
}
