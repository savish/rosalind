//! DNA module
//!
//! This module contains the types used to create and manipulate `DNA` and `RNA` strings.

use std::fmt;

// ///// //
// Types //
// ///// //

/// List of symbols present in a DNA string
pub const DNA_SYMBOLS: [char; 4] = ['A', 'C', 'G', 'T'];

/// List of symbols present in an RNA string
pub const RNA_SYMBOLS: [char; 4] = ['A', 'C', 'G', 'U'];

/// Defines behaviours for genetic strings
pub trait GeneticString {
    fn content(&self) -> &str;

    fn length(&self) -> usize {
        self.content().chars().count()
    }

    fn gc_content(&self) -> f64 {
        let gc =
            (count_character('G', &self.content()) + count_character('C', &self.content())) as i32;
        let dna_len = self.length() as i32;
        (gc as f64 / dna_len as f64) * 100f64
    }
    // fn symbol_count(&self) -> Vec<usize>;
}

/// Represents a strand of DNA
pub struct DNA(String);

/// Represents a strand of RNA
pub struct RNA(String);

/// Represents a Protein string formed from RNA strands
pub struct Protein(String);

// Represents a FASTA format labeled string
pub struct FASTA {
    // Can be very long
    content: Box<GeneticString + 'static>,
    // Generally small enough to clone
    label: String,
}

// /////////////// //
// Implementations //
// /////////////// //

// DNA
// --

impl DNA {
    pub fn new(dna_string: &str) -> DNA {
        DNA(String::from(dna_string.trim()))
    }

    /// Get the reverse complement of a DNA strand
    ///
    /// # Example
    ///
    /// ```
    /// use dna::*;
    ///
    /// let dna = DNA::new("AACGGT");
    /// assert_eq!(dna.reverse_complement().content(), DNA::new("ACCGTT").content());
    /// ```
    pub fn reverse_complement(&self) -> DNA {
        let DNA(ref dna_string) = *self;
        DNA(reverse_string(&dna_string)
            .chars()
            .map(|symbol| DNA::complement(symbol))
            .collect::<String>())
    }

    /// Counts the number of times each DNA symbol appears in a DNA string
    ///
    /// # Example
    ///
    /// ```
    /// use dna::*;
    /// let dna = DNA::new("ACGT");
    /// assert_eq!(dna.count_symbols(), vec![1, 1, 1, 1]);
    /// ```
    pub fn count_symbols(&self) -> Vec<usize> {
        DNA_SYMBOLS
            .iter()
            .map(|symbol| count_character(*symbol, self.content()))
            .collect::<Vec<_>>()
    }

    fn complement(symbol: char) -> char {
        match symbol {
            'A' => 'T',
            'C' => 'G',
            'G' => 'C',
            'T' => 'A',
            _ => panic!("Invalid DNA string"),
        }
    }
}

impl GeneticString for DNA {
    fn content(&self) -> &str {
        let DNA(ref content) = *self;
        content
    }
}

impl From<RNA> for DNA {
    fn from(rna: RNA) -> Self {
        let RNA(ref rna_string) = rna;

        let dna_string = rna_string
            .chars()
            .map(|symbol| get_dna_symbol(symbol))
            .collect::<String>();

        DNA::new(&dna_string)
    }
}

impl fmt::Display for DNA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.content())
    }
}

// RNA
// --

impl RNA {
    pub fn new(rna_string: &str) -> RNA {
        RNA(String::from(rna_string.trim()))
    }
}

impl GeneticString for RNA {
    fn content(&self) -> &str {
        let RNA(ref content) = *self;
        content
    }
}

impl From<DNA> for RNA {
    fn from(dna: DNA) -> Self {
        let DNA(ref dna_string) = dna;

        let rna_string = dna_string
            .chars()
            .map(|symbol| get_rna_symbol(symbol))
            .collect::<String>();

        RNA::new(&rna_string)
    }
}

impl fmt::Display for RNA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.content())
    }
}

// Protein
// --

impl Protein {
    pub fn new(protein_string: &str) -> Protein {
        Protein(String::from(protein_string.trim()))
    }
}

impl GeneticString for Protein {
    fn content(&self) -> &str {
        let Protein(ref content) = *self;
        content
    }
}

impl From<RNA> for Protein {
    fn from(rna: RNA) -> Self {
        let RNA(ref rna_string) = rna;

        let rna_chars: Vec<char> = rna_string.chars().collect();
        let string_arr = &rna_chars
            .chunks(3)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<_>>();

        let p_string = string_arr
            .iter()
            .map(|cd| codon_table(&cd))
            .collect::<Vec<_>>();

        Protein::new(&p_string.join(""))
    }
}

impl fmt::Display for Protein {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.content())
    }
}

// FASTA
// --

impl FASTA {
    // Creates a new FASTA format lebeled string
    pub fn new<T: GeneticString + 'static>(gen_string: T, label: &str) -> FASTA {
        FASTA {
            content: Box::new(gen_string),
            label: String::from(label),
        }
    }

    pub fn label(&self) -> String {
        self.label.clone()
    }

    pub fn content(&self) -> &str {
        (*self.content).content()
    }

    pub fn gc_content(&self) -> f64 {
        (*self.content).gc_content()
    }
}

// ///////// //
// Functions //
// ///////// //

/// Compute the Hamming distance between two strings
pub fn hamming_distance(first: &str, other: &str) -> usize {
    first
        .chars()
        .zip(other.chars())
        .filter(|pair| pair.0 != pair.1)
        .count()
}

// Count the number of times a character occurs in the given string
fn count_character(character: char, in_string: &str) -> usize {
    in_string
        .chars()
        .filter(|ch| *ch == character)
        .collect::<Vec<_>>()
        .len()
}

// Reverse a given string
fn reverse_string(input: &str) -> String {
    input.chars().rev().collect::<String>()
}

// Return the RNA symbol that corresponds to the given DNA symbol
fn get_rna_symbol(symbol: char) -> char {
    RNA_SYMBOLS[DNA_SYMBOLS.iter().position(|&x| x == symbol).unwrap()]
}

// Return the DNA symbol that corresponds to the given RNA symbol
fn get_dna_symbol(symbol: char) -> char {
    DNA_SYMBOLS[RNA_SYMBOLS.iter().position(|&x| x == symbol).unwrap()]
}

// Return the protein string produced by the given RNA strand
fn codon_table(rna_slice: &str) -> &str {
    match rna_slice {
        "GAU" => "D",
        "GAC" => "D",
        "GAA" => "E",
        "GAG" => "E",
        "GGU" => "G",
        "GGC" => "G",
        "GGA" => "G",
        "GGG" => "G",

        "GUU" => "V",
        "GUC" => "V",
        "GUA" => "V",
        "GUG" => "V",
        "GCU" => "A",
        "GCC" => "A",
        "GCA" => "A",
        "GCG" => "A",

        "AAU" => "N",
        "AAC" => "N",
        "AAA" => "K",
        "AAG" => "K",
        "AGU" => "S",
        "AGC" => "S",
        "AGA" => "R",
        "AGG" => "R",

        "AUU" => "I",
        "AUC" => "I",
        "AUA" => "I",
        "AUG" => "M",
        "ACU" => "T",
        "ACC" => "T",
        "ACA" => "T",
        "ACG" => "T",

        "CAU" => "H",
        "CAC" => "H",
        "CAA" => "Q",
        "CAG" => "Q",
        "CGU" => "R",
        "CGC" => "R",
        "CGA" => "R",
        "CGG" => "R",

        "CUU" => "L",
        "CUC" => "L",
        "CUA" => "L",
        "CUG" => "L",
        "CCU" => "P",
        "CCC" => "P",
        "CCA" => "P",
        "CCG" => "P",

        "UUU" => "F",
        "UUC" => "F",
        "UUA" => "L",
        "UUG" => "L",
        "UCU" => "S",
        "UCC" => "S",
        "UCA" => "S",
        "UCG" => "S",

        "UAU" => "Y",
        "UAC" => "Y",
        "UAA" => "",
        "UAG" => "",
        "UGU" => "C",
        "UGC" => "C",
        "UGA" => "",
        "UGG" => "W",

        _ => "",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_counts_individual_symbols() {
        let dna = DNA::new("AACGGGTTTT");
        assert_eq!(count_character('A', dna.content()), 2);
        assert_eq!(count_character('C', dna.content()), 1);
        assert_eq!(count_character('G', dna.content()), 3);
        assert_eq!(count_character('T', dna.content()), 4);
    }

    #[test]
    fn it_gets_complement_symbols() {
        assert_eq!(DNA::complement('G'), 'C');
        assert_eq!(DNA::complement('A'), 'T');
        assert_eq!(DNA::complement('C'), 'G');
        assert_eq!(DNA::complement('T'), 'A');
    }

    #[test]
    #[should_panic(expected = "Invalid DNA string")]
    fn it_only_complements_valid_symbols() {
        assert_eq!(DNA::complement('Z'), 'Y');
    }
}
