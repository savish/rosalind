extern crate gen_str;

use gen_str::*;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut f = File::open(filename).expect("file not found");

    let mut rna_string = String::new();
    f.read_to_string(&mut rna_string)
        .expect("something went wrong reading the file");

    println!("{}", Protein::from(RNA::new(&rna_string)))
}
