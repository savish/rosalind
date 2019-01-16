#[macro_use]
extern crate clap;
extern crate fib;
extern crate gen_str;
extern crate perm;
extern crate rosalind;

use clap::App;

fn main() -> Result<(), Box<std::error::Error>> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.subcommand() {
        ("dna", Some(dna_matches)) => runners::dna(dna_matches.value_of("dna_string").unwrap()),
        ("rna", Some(rna_matches)) => runners::rna(rna_matches.value_of("dna_string").unwrap()),
        ("revc", Some(revc_matches)) => runners::revc(revc_matches.value_of("dna_string").unwrap()),
        ("prot", Some(prot_matches)) => runners::prot(prot_matches.value_of("rna_file").unwrap()),
        ("gc", Some(gc_matches)) => runners::gc(gc_matches.value_of("dna_file").unwrap()),
        ("fib", Some(fib_matches)) => runners::fib(
            fib_matches.value_of("months").unwrap().parse::<u8>()?,
            fib_matches.value_of("pairs").unwrap().parse::<u8>()?,
        ),
        ("fibd", Some(fibd_matches)) => runners::fibd(
            fibd_matches.value_of("months").unwrap().parse::<u8>()?,
            fibd_matches
                .value_of("life_expectancy")
                .unwrap()
                .parse::<u8>()?,
        ),
        ("hamm", Some(hamm_matches)) => runners::hamm(
            hamm_matches.value_of("string_1").unwrap(),
            hamm_matches.value_of("string_2").unwrap(),
        ),
        ("perm", Some(perm_matches)) => runners::perm(
            perm_matches
                .value_of("permutation_length")
                .unwrap()
                .parse::<u8>()?,
        ),
        ("sign", Some(sign_matches)) => runners::sign(
            sign_matches
                .value_of("permutation_length")
                .unwrap()
                .parse::<u8>()?,
        ),
        ("subs", Some(subs_matches)) => runners::subs(
            subs_matches.value_of("dna_string").unwrap(),
            subs_matches.value_of("substring").unwrap(),
        ),
        ("mrna", Some(mrna_matches)) => {
            runners::mrna(mrna_matches.value_of("protein_string").unwrap())
        }
        ("", None) => println!("No subcommand was used"),
        _ => unreachable!(),
    }

    Ok(())
}

mod runners {
    use fib::*;
    use gen_str::*;
    use perm::*;
    use rosalind::*;
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
            .filter(|fdna| !fdna.is_empty())
            .map(|fdna| fdna_from_string(fdna))
            .collect();

        fdna_array.iter().for_each(|fdna| {
            println!("{}", fdna.label());
            println!("{}", fdna.gc_content())
        });
    }

    pub fn fib(months: u8, pairs: u8) {
        println!(
            "{:?}\n",
            population(pairs as usize)
                .nth((months - 1) as usize)
                .unwrap()
        );
    }

    pub fn fibd(months: u8, life_expectancy: u8) {
        println!(
            "{:?}\n",
            population_with_moratilty(1, life_expectancy as usize)
                .nth((months - 1) as usize)
                .unwrap()
        );
    }

    pub fn hamm(string_1: &str, string_2: &str) {
        println!("{}", hamming_distance(string_1, string_2));
    }

    pub fn perm(permutation_length: u8) {
        // TODO: writeln! + stdout lock
        println!("{}", factorial(u64::from(permutation_length)));
        for code in permutations((1i64..=i64::from(permutation_length)).collect::<Vec<_>>()) {
            println!("{}", code);
        }
    }

    pub fn sign(permutation_length: u8) {
        // TODO: writeln! + stdout lock
        let permutation_length_pow2 = 2u64.pow(u32::from(permutation_length));

        // Number of outputs
        println!(
            "{}",
            factorial(u64::from(permutation_length)) * permutation_length_pow2
        );

        // Permutations
        for code in permutations((1i64..=i64::from(permutation_length)).collect::<Vec<_>>()) {
            let vec = &*code; // Deref from wrapper
            for binary in 0..permutation_length_pow2 {
                let binary = generate_binary(binary, permutation_length as usize);
                let zipped = binary.iter().zip(vec.iter()).collect::<Vec<_>>();
                let perm = zipped
                    .into_iter()
                    .map(|val| *val.0 * *val.1 as i64)
                    .collect::<Vec<_>>();
                println!("{}", VecWrapper::new(perm));
            }
        }
    }

    pub fn subs(dna_string: &str, substring: &str) {
        println!(
            "{:?}",
            substring_locations(dna_string, substring)
                .iter()
                .map(|x| (x + 1).to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
    }

    pub fn mrna(protein_string: &str) {
        println!(
            "{}",
            Protein::new(protein_string)
                .rna_count(1_000_000)
                .remainder()
        );
    }
}
