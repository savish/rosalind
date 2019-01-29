//! # Rosalind project
//!
//! The Rosalind project is a platform for learning bioinformatics through problem solving.
//! More information about the project is available at http://rosalind.info/about/
//!
//! This repository contains solutions to some of the rosalind problems. Each propblem is available as a subcommand under the main executable.
//!
//! ## 1. Usage
//!
//! By default, this application compiles to a binary called `rosalind`. Sample usage is given in the code snippet below:
//!
//! ```bash
//! # Shows the available subcommands
//! $ rosalind --help
//!
//! rosalind 0.1.0
//! Alan K <afksavish@gmail.com>
//! Solutions to the Rosalind Project problem set
//!
//! USAGE:
//!     rosalind <SUBCOMMAND>
//!
//! FLAGS:
//!     -h, --help       Prints help information
//!     -V, --version    Prints version information
//!
//! SUBCOMMANDS:
//!     dna     Counting DNA Nucleotides
//!     fib     Rabbits and Recurrence Relations
//!     fibd    Mortal Fibonacci Rabbits
//!     gc      Computing GC Content
//!     hamm    Counting Point Mutations
//!     help    Prints this message or the help of the given subcommand(s)
//!     iprb    Introduction to Mendelian Inheritance
//!     mrna    Inferring mRNA from Protein
//!     perm    Enumerating Gene Orders
//!     prot    Translating RNA into Protein
//!     revc    Complementing a Strand of DNA
//!     rna     Transcribing DNA into RNA
//!     sign    Enumerating Oriented Gene Orderings
//!     subs    Finding a Motif in DNA
//! ```
//!
//! ### Example subcommand help
//!
//! ```bash
//! $ rosalind revc --help
//!
//! rosalind-revc
//! Complementing a Strand of DNA
//!
//! USAGE:
//!     rosalind revc <dna_string>
//!
//! FLAGS:
//!     -h, --help       Prints help information
//!     -V, --version    Prints version information
//!
//! ARGS:
//!     <dna_string>    A DNA string, upto 1000 nucleotides in length.
//! ```
//!
//! ### Example subcommand
//!
//! ```bash
//! $ rosalind revc ACGTACGTC
//! GACGTACGT
//! ```
//!
//! ## 2. Setup
//!
//! This project is written entirely in Rust (edition 2018).You can get rust
//! from here https://www.rust-lang.org
//!
//! Once you have a working installation of rust available,
//! proceed as follows from a terminal session:
//!
//! ```bash
//! $ cd <SOME_WORKING_DIR>
//! $ git clone <THIS_REPO>
//! $ cd rosalind
//! $ cargo build
//! ```
//!
//! ## 3 Testing
//!
//! From within the `rosalind` repo directory use:
//!
//! ```bash
//! $ cargo test
//! ```
//!

pub mod fib;
pub mod gen_str;
pub mod gene;
pub mod perm;

// //////// //
// Funtions //
// //////// //

/// Compute the Hamming distance between two strings
///
/// The hamming distance represents the number of single-character changes required to convert one
/// string to another.
///
/// # Example
/// ```rust
/// # use rosalind::hamming_distance;
/// hamming_distance("ACGTACGTAC", "AGGTACGTAA"); // 2
/// # assert_eq!(hamming_distance("ACGTACGTAC", "AGGTACGTAA"), 2);
/// ```
pub fn hamming_distance(first: &str, other: &str) -> usize {
    first
        .chars()
        .zip(other.chars())
        .filter(|pair| pair.0 != pair.1)
        .count()
}

/// Determine the positions of a substring in a given string
///
/// Returns a list of indices representing the starting position of each occurence of the substring
///
/// # Example
/// ```rust
/// # use rosalind::substring_locations;
/// let source_string = "GATATATGCATATACTT";
/// let substring = "ATAT";
/// substring_locations(source_string, substring);  // [1, 3, 9];
/// # assert_eq!(substring_locations(source_string, substring), vec![1usize, 3, 9]);
/// ```
pub fn substring_locations(source_string: &str, substring: &str) -> Vec<usize> {
    let mut locations: Vec<usize> = vec![];
    get_substring_locations(
        source_string.trim(),
        substring.trim(),
        &mut locations,
        0usize,
    )
}

fn get_substring_locations(
    source_string: &str,
    substring: &str,
    locations: &mut Vec<usize>,
    slice_offset: usize,
) -> Vec<usize> {
    match source_string.find(substring) {
        Some(location) => {
            locations.push(location + slice_offset);
            get_substring_locations(
                &source_string[(location + 1)..],
                substring,
                locations,
                location + slice_offset + 1,
            )
        }
        None => locations.clone(),
    }
}
