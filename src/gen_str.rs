//! Genetic strings module
//!
//! This module contains the types used to create and manipulate genetic strings. Genetic strings
//! are representations of the following biological structures:
//!
//! - DNA strands, encoded with genetic information
//! - RNA strands, used as messengers of genetic information
//! - Protein strings which are formed from RNA strands
//!
//! These strings can be labelled. The labelling format used in this project is the FASTA format,
//! which uses whitespace to separate labels from strands.

use std::fmt;
use std::ops::Mul;

// ///// //
// Types //
// ///// //

/// List of symbols present in a DNA strand
pub const DNA_SYMBOLS: [char; 4] = ['A', 'C', 'G', 'T'];

/// List of symbols present in an RNA strand
pub const RNA_SYMBOLS: [char; 4] = ['A', 'C', 'G', 'U'];

/// Defines behaviours for genetic strings
pub trait GeneticString {
    /// Return the content of a genetic string.
    ///
    /// This would be the actual string representation of the strand. Primarily, this is used to
    /// extract the genetic string content of a FASTA labelled string
    ///
    /// # Example
    /// ```
    /// # use rosalind::gen_str::*;
    /// let dna = DNA::new("ACGT");
    /// dna.content();  // "ACGT"
    /// # assert_eq!(dna.content(), "ACGT")
    /// ```
    fn content(&self) -> &str;

    /// Return the length of a genetic string
    ///
    /// The length does not include the FASTA label if present, only the content string.
    ///
    /// # Example
    /// ```rust
    /// # use rosalind::gen_str::*;
    /// let dna = DNA::new("ACGTACGT");
    /// dna.length();  // 8
    /// # assert_eq!(dna.length(), 8)
    /// ```
    fn length(&self) -> usize {
        self.content().chars().count()
    }

    /// Compute the GC content of a genetic string
    ///
    /// The GC content is the proportion of `G` and `C` characters in a strand. As such, it is only
    /// useful for RNA and DNA strands, with no meaning for protein strings. A strand with a high
    /// GC content is considered more stable, due to the `GC` bond being stronger than the other
    /// bonds.
    ///
    /// # Example
    /// ```rust
    /// # use rosalind::gen_str::*;
    /// let dna = DNA::new("ACGTCGCGTA");
    /// dna.gc_content();  // 60.0 (percentage)
    /// # assert_eq!(dna.gc_content(), 60f64);
    /// ```
    fn gc_content(&self) -> f64 {
        let gc =
            (count_character('G', &self.content()) + count_character('C', &self.content())) as i32;
        let dna_len = self.length() as i32;
        (f64::from(gc) / f64::from(dna_len)) * 100f64
    }
    // fn symbol_count(&self) -> Vec<usize>;
}

/// Represents a strand of DNA
pub struct DNA(String);

/// Represents a strand of RNA
pub struct RNA(String);

/// Represents a Protein string formed from RNA strands
pub struct Protein(String);

/// Represents a FASTA format labelled string
///
/// ```text
/// >DNA_1
/// AACCGGTTTGGCACGTACGTACTACTACGATCGTAGCTACG
/// ACGTACGTACTA
/// >DNA_2
/// GATCGATCTACGTAGCTAGCTGATCGTAGCTGCTGACTGAT
/// ACGTACGTGAC
/// ```
///
/// # Example
/// ```rust
/// # use rosalind::gen_str::*;
/// let fasta = FASTA::new(DNA::new("ACGT"), "DNA_1");
/// fasta.label();      // "DNA_1"
/// # assert_eq!(fasta.label(), "DNA_1");
/// fasta.content();    // "ACGT"
/// # assert_eq!(fasta.content(), "ACGT");
/// fasta.length();     // 4
/// # assert_eq!(fasta.length(), 4);
/// ```
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
    /// Initialize and return a new DNA struct
    ///
    /// # Example
    /// ```rust
    /// # use rosalind::gen_str::DNA;
    /// let dna = DNA::new("ACGT");
    /// ```
    pub fn new(dna_string: &str) -> DNA {
        DNA(String::from(dna_string.trim()))
    }

    /// Compute and return the reverse complement of a DNA strand
    ///
    /// # Example
    ///
    /// ```
    /// # use rosalind::gen_str::*;
    /// let dna = DNA::new("AACGGT");
    /// dna.reverse_complement().content(); // "ACCGTT"
    /// # assert_eq!(dna.reverse_complement().content(), DNA::new("ACCGTT").content());
    /// ```
    pub fn reverse_complement(&self) -> DNA {
        let DNA(ref dna_string) = *self;
        DNA(reverse_string(&dna_string)
            .chars()
            .map(DNA::complement)
            .collect::<String>())
    }

    /// Count the number of times each DNA symbol appears in a DNA string
    ///
    /// The resulting vector is in the order `[A, C, G, T]`
    ///
    /// # Example
    ///
    /// ```
    /// # use rosalind::gen_str::*;
    /// let dna = DNA::new("ACGGTAAC");
    /// dna.count_symbols(); // [3, 2, 2, 1] (Vector)
    /// # assert_eq!(dna.count_symbols(), vec![3, 2, 2, 1]);
    /// ```
    pub fn count_symbols(&self) -> Vec<usize> {
        DNA_SYMBOLS
            .iter()
            .map(|symbol| count_character(*symbol, self.content()))
            .collect::<Vec<_>>()
    }

    // Return the complement for each DNA character
    fn complement(symbol: char) -> char {
        DNA_SYMBOLS[DNA_SYMBOLS
            .iter()
            .rev()
            .position(|&x| x == symbol)
            .expect("Invalid DNA string")]
    }
}

impl GeneticString for DNA {
    fn content(&self) -> &str {
        let DNA(ref content) = *self;
        content
    }
}

impl From<RNA> for DNA {
    /// Convert an RNA strand into a DNA strand
    ///
    /// # Example
    /// ```rust
    /// # use rosalind::gen_str::*;
    /// let rna = RNA::new("ACGUUGCA");
    /// let dna = DNA::from(rna);  // "ACGTTGCA"
    /// # assert_eq!(dna.content(), "ACGTTGCA");
    /// ```
    fn from(rna: RNA) -> Self {
        let RNA(ref rna_string) = rna;

        let dna_string = rna_string.chars().map(get_dna_symbol).collect::<String>();

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
    /// Initialize and return a new RNA struct
    ///
    /// # Example
    /// ```rust
    /// # use rosalind::gen_str::RNA;
    /// let rna = RNA::new("ACGU");
    /// ```
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
    /// Convert a DNA strand into an RNA strand
    ///
    /// # Example
    /// ```rust
    /// # use rosalind::gen_str::*;
    /// let dna = DNA::new("CGTACGATCG");
    /// let rna = RNA::from(dna);  // "CGUACGAUCG"
    /// # assert_eq!(rna.content(), "CGUACGAUCG");
    /// ```
    fn from(dna: DNA) -> Self {
        let DNA(ref dna_string) = dna;

        let rna_string = dna_string.chars().map(get_rna_symbol).collect::<String>();

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
    /// Initialize and return a new Protein string
    ///
    /// # Example
    /// ```rust
    /// # use rosalind::gen_str::Protein;
    /// let protein = Protein::new("MTSMSS");
    /// ```
    pub fn new(protein_string: &str) -> Protein {
        Protein(String::from(protein_string.trim()))
    }

    /// Determine the number of possible RNA strands that would form this protein string
    ///
    /// # Example
    /// ```rust
    /// # use rosalind::gen_str::*;
    /// let protein = Protein::new("MA");
    /// assert_eq!(protein.rna_count(1000), Modulo::new(12, 1000));
    /// ```
    pub fn rna_count(&self, modulo: u32) -> Modulo {
        self.content().chars().fold(
            Modulo::from(rna_codon(' ').len() as u32, modulo),
            |acc, ch| acc * Modulo::from(rna_codon(ch).len() as u32, modulo),
        )
    }
}

impl GeneticString for Protein {
    fn content(&self) -> &str {
        let Protein(ref content) = *self;
        content
    }
}

impl From<RNA> for Protein {
    /// Convert an RNA strand into a Protein string
    ///
    /// This is a one-way conversion due to the requirement that the RNA strand be divided into
    /// chunks of 3. If the strand is not divisible by 3, the remaining characters are ignored.
    /// Therefore, converting backwards from a protein string into an RNA strand may lack up to 2
    /// characters that were present in the original RNA strand
    ///
    /// # Example
    /// ```rust
    /// # use rosalind::gen_str::*;
    /// let rna = RNA::new("AAGUGUCUGGCUUGAAGU");
    /// let protein = Protein::from(rna);  // "KCLAS"
    /// # assert_eq!(protein.content(), "KCLAS");
    /// ```
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
    /// Initialize and return a new FASTA labelled genetic string
    ///
    /// # Example
    /// ```rust
    /// # use rosalind::gen_str::*;
    /// let fasta = FASTA::new(DNA::new("ACGTTGCATC"), "DNA_1");
    /// ```
    pub fn new<T: GeneticString + 'static>(gen_string: T, label: &str) -> FASTA {
        FASTA {
            content: Box::new(gen_string),
            label: String::from(label),
        }
    }

    /// Get the label of this FASTA string
    ///
    /// # Example
    /// ```rust
    /// # use rosalind::gen_str::*;
    /// let fasta = FASTA::new(DNA::new("ACGTTGCATC"), "DNA_1");
    /// fasta.label(); // "DNA_1"
    /// # assert_eq!(fasta.label(), "DNA_1");
    /// ```
    pub fn label(&self) -> String {
        self.label.clone()
    }
}

impl GeneticString for FASTA {
    fn content(&self) -> &str {
        (*self.content).content()
    }
}

// ///////// //
// Functions //
// ///////// //

// Count the number of times a character occurs in the given string
fn count_character(character: char, in_string: &str) -> usize {
    in_string.chars().filter(|ch| *ch == character).count()
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
        "GGU" | "GGC" | "GGA" | "GGG" => "G",
        "GUU" | "GUC" | "GUA" | "GUG" => "V",
        "GCU" | "GCC" | "GCA" | "GCG" => "A",
        "ACG" | "ACA" | "ACC" | "ACU" => "T",
        "CGG" | "CGA" | "CGC" | "CGU" | "AGG" | "AGA" => "R",
        "CUG" | "CUA" | "CUC" | "CUU" | "UUG" | "UUA" => "L",
        "CCG" | "CCA" | "CCC" | "CCU" => "P",
        "UCG" | "UCA" | "UCC" | "UCU" | "AGC" | "AGU" => "S",
        "AUA" | "AUC" | "AUU" => "I",
        "UAG" | "UGA" | "UAA" => "",
        "GAU" | "GAC" => "D",
        "GAA" | "GAG" => "E",
        "AAU" | "AAC" => "N",
        "AAA" | "AAG" => "K",
        "CAC" | "CAU" => "H",
        "CAG" | "CAA" => "Q",
        "UUC" | "UUU" => "F",
        "UAC" | "UAU" => "Y",
        "UGC" | "UGU" => "C",
        "AUG" => "M",
        "UGG" => "W",
        _ => "",
    }
}

fn rna_codon(amino_acid: char) -> Vec<&'static str> {
    match amino_acid {
        'A' => vec!["GCU", "GCC", "GCA", "GCG"],
        'C' => vec!["UGC", "UGU"],
        'D' => vec!["GAU", "GAC"],
        'E' => vec!["GAA", "GAG"],
        'F' => vec!["UUC", "UUU"],
        'G' => vec!["GGU", "GGC", "GGA", "GGG"],
        'H' => vec!["CAC", "CAU"],
        'I' => vec!["AUA", "AUC", "AUU"],
        'K' => vec!["AAA", "AAG"],
        'L' => vec!["UUG", "UUA", "CUG", "CUA", "CUC", "CUU"],
        'M' => vec!["AUG"],
        'N' => vec!["AAU", "AAC"],
        'P' => vec!["CCG", "CCA", "CCC", "CCU"],
        'Q' => vec!["CAG", "CAA"],
        'R' => vec!["AGG", "AGA", "CGG", "CGA", "CGC", "CGU"],
        'S' => vec!["AGC", "AGU", "UCG", "UCA", "UCC", "UCU"],
        'T' => vec!["ACG", "ACA", "ACC", "ACU"],
        'V' => vec!["GUU", "GUC", "GUA", "GUG"],
        'W' => vec!["UGG"],
        'Y' => vec!["UAC", "UAU"],
        ' ' => vec!["UAG", "UGA", "UAA"],
        _ => panic!("Invalid amino acid"),
    }
}

// ///// //
// Types //
// ///// //

/// Holds the modulo representation of a number
///
/// In mathematics, `a` modulo `n` is the remainder obtained when `a` is divided by `n`. For
/// instance **32** `mod` **6** = **2**
///
/// This struct stores the dividend and the remainder resulting from the above division. Therefore,
/// based on the above calculation, the number **32** can be converted into `Modulo(2, 6)` where
/// **2** is the remainder and **6** is the dividend.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Modulo {
    remainder: u32,
    dividend: u32,
}

impl Modulo {
    /// Return a new `Modulo` instance
    ///
    /// # Example
    /// ```rust
    /// # use rosalind::gen_str::Modulo;
    /// let modulo = Modulo::new(3, 5);
    /// # assert_eq!(modulo.remainder(), 3);
    /// # assert_eq!(modulo.dividend(), 5);
    pub fn new(remainder: u32, dividend: u32) -> Modulo {
        Modulo {
            remainder,
            dividend,
        }
    }

    /// Create a `Modulo` number from a regular one
    ///
    /// # Example
    /// ```rust
    /// # use rosalind::gen_str::Modulo;
    /// let modulo = Modulo::from(32, 5);  // Modulo { 2, 5 }
    /// # assert_eq!(modulo, Modulo::new(2, 5));
    /// ```
    pub fn from(number: u32, dividend: u32) -> Modulo {
        Modulo {
            remainder: number % dividend,
            dividend,
        }
    }

    /// Returns the remainder part of a modulo number
    pub fn remainder(self) -> u32 {
        self.remainder
    }

    /// Returns the dividend part of a modulo number
    pub fn dividend(self) -> u32 {
        self.dividend
    }
}

impl Mul for Modulo {
    type Output = Self;

    /// Multiplies two `Modulo` numbers
    ///
    /// # Example
    ///
    /// ```rust
    /// # use rosalind::gen_str::Modulo;
    /// let mod1 = Modulo::from(23, 4);
    /// let mod2 = Modulo::from(67, 4);
    ///
    /// assert_eq!(mod1 * mod2, Modulo::new(1, 4));
    /// ```
    /// Both numbers should have the same dividend to be multiplied. If not, the code should panic.
    /// For instance, the code block below will panic.
    ///
    /// ```rust,should_panic
    /// # use rosalind::gen_str::Modulo;
    /// let mod1 = Modulo::from(23, 6);
    /// let mod2 = Modulo::from(67, 5);
    ///
    /// assert_eq!(mod1 * mod2, Modulo::new(4, 6));
    /// ```
    fn mul(self, rhs: Self) -> Self {
        if self.dividend() != rhs.dividend() {
            panic!("Multiplication is only valid for modulo numbers with the same dividend")
        }

        Modulo::from(self.remainder() * rhs.remainder(), self.dividend())
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
