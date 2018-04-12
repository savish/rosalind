extern crate gen_str;

use gen_str::*;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn fdna_from_string(fdna: &str) -> FASTA {
    let mut lines = fdna.split_whitespace();
    let label = lines.next().unwrap().to_string();
    let dna_string = lines.collect::<Vec<&str>>().join("");

    FASTA::new(DNA::new(&dna_string), &label)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut f = File::open(filename).expect("file not found");

    let mut fasta_dna_strings = String::new();
    f.read_to_string(&mut fasta_dna_strings)
        .expect("something went wrong reading the file");

    let fdna_array: Vec<FASTA> = fasta_dna_strings
        .split('>')
        .filter(|fdna| fdna.len() > 0)
        .map(|fdna| fdna_from_string(fdna))
        .collect();

    fdna_array
        .iter()
        .map(|fdna| {
            println!("{}", fdna.label());
            println!("{}", fdna.gc_content())
        })
        .collect::<Vec<_>>();
}
