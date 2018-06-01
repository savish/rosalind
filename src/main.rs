#[macro_use]
extern crate clap;
extern crate gen_str;

use clap::App;
use gen_str::{DNA, RNA};
use std::io;

fn main() -> io::Result<()> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.subcommand() {
        ("dna", Some(dna_matches)) => {
            let dna = DNA::new(dna_matches.value_of("dna_string").unwrap());
            println!(
                "{} {} {} {}",
                dna.count_symbols()[0],
                dna.count_symbols()[1],
                dna.count_symbols()[2],
                dna.count_symbols()[3]
            );
        }
        ("rna", Some(rna_matches)) => {
            println!(
                "{}",
                RNA::from(DNA::new(rna_matches.value_of("rna_string").unwrap()))
            );
        }
        ("", None) => println!("No subcommand was used"),
        _ => unreachable!(),
    }

    Ok(())
}
