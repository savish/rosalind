#[macro_use]
extern crate clap;
extern crate gen_str;
use clap::App;
use gen_str::*;
use std::io;

fn main() -> io::Result<()> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.subcommand() {
        ("dna", Some(dna_matches)) => runners::dna(dna_matches.value_of("dna_string").unwrap()),
        ("rna", Some(rna_matches)) => runners::rna(rna_matches.value_of("dna_string").unwrap()),
        ("revc", Some(revc_matches)) => runners::revc(revc_matches.value_of("dna_string").unwrap()),
        ("prot", Some(prot_matches)) => runners::prot(prot_matches.value_of("rna_file").unwrap()),
        ("gc", Some(gc_matches)) => runners::gc(gc_matches.value_of("dna_file").unwrap()),
        ("", None) => println!("No subcommand was used"),
        _ => unreachable!(),
    }

    Ok(())
}

mod runners {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    fn fdna_from_string(fdna: &str) -> FASTA {
        let mut lines = fdna.split_whitespace();
        let label = lines.next().unwrap().to_string();
        let dna_string = lines.collect::<Vec<&str>>().join("");

        FASTA::new(DNA::new(&dna_string), &label)
    }

    pub fn dna(dna_string: &str) {
        let dna = DNA::new(dna_string);
        println!(
            "{} {} {} {}",
            dna.count_symbols()[0],
            dna.count_symbols()[1],
            dna.count_symbols()[2],
            dna.count_symbols()[3]
        );
    }

    pub fn rna(dna_string: &str) {
        println!("{}", RNA::from(DNA::new(dna_string)));
    }

    pub fn revc(dna_string: &str) {
        println!("{}", DNA::new(dna_string).reverse_complement());
    }

    pub fn prot(rna_file_name: &str) {
        let mut f = File::open(rna_file_name).expect("file not found");

        let mut rna_string = String::new();
        f.read_to_string(&mut rna_string)
            .expect("something went wrong reading the file");

        println!("{}", Protein::from(RNA::new(&rna_string)))
    }

    pub fn gc(dna_file_name: &str) {
        let mut f = File::open(dna_file_name).expect("file not found");

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
}
