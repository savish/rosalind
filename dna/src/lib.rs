//! DNA module

use std::fmt;

/// List of symbols present in a DNA string
pub const DNA_SYMBOLS: [char; 4] = ['A', 'C', 'G', 'T'];

/// List of symbols present in an RNA string
pub const RNA_SYMBOLS: [char; 4] = ['A', 'C', 'G', 'U'];

// Reverse a given string
fn reverse_string(input: &str) -> String {
    input.chars().rev().collect::<String>()
}

/// Represents a single strand of DNA
#[derive(Debug, PartialEq)]
pub struct DNA(String);

impl fmt::Display for DNA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let DNA(ref dna_string) = *self;
        write!(f, "{}", dna_string)
    }
}

impl DNA {
    // Counts the number of times a given symbol appears in a given DNA string
    fn count_symbol(&self, symbol: char) -> usize {
        let DNA(ref dna_string) = *self;

        dna_string
            .chars()
            .filter(|ch| *ch == symbol)
            .collect::<Vec<_>>()
            .len()
    }

    // Get the base-pair complement of a DNA symbol
    fn complement(symbol: char) -> char {
        match symbol {
            'A' => 'T',
            'C' => 'G',
            'G' => 'C',
            'T' => 'A',
            _ => panic!("Invalid DNA string"),
        }
    }

    // Get the equivalent RNA symbol for a DNA one
    fn get_rna_symbol(symbol: char) -> char {
        RNA_SYMBOLS[DNA_SYMBOLS.iter().position(|&x| x == symbol).unwrap()]
    }

    /// Create a new DNA strand
    ///
    /// # Example
    ///
    /// ```
    /// use dna::*;
    ///
    /// let dna = DNA::new("ACGT");
    /// assert_eq!(dna, DNA::new("ACGT"));
    /// ```
    pub fn new(dna_string: &str) -> DNA {
        DNA(dna_string.trim().to_string())
    }

    /// Get the reverse complement of a DNA strand
    ///
    /// # Example
    ///
    /// ```
    /// use dna::*;
    ///
    /// let dna = DNA::new("AACGGT");
    /// assert_eq!(dna.reverse_complement(), DNA::new("ACCGTT"));
    /// ```
    pub fn reverse_complement(&self) -> DNA {
        let DNA(ref dna_string) = *self;
        DNA(
            reverse_string(&dna_string)
                .chars()
                .map(|symbol| DNA::complement(symbol))
                .collect::<String>(),
        )
    }

    /// Convert the DNA strand into an RNA strand
    ///
    /// # Example
    ///
    /// ```
    /// use dna::*;
    ///
    /// let dna = DNA::new("AACGGT");
    /// assert_eq!(dna.to_rna(), RNA::new("AACGGU"));
    /// ```
    pub fn to_rna(&self) -> RNA {
        let DNA(ref dna_string) = *self;

        let rna_string = dna_string
            .chars()
            .map(|symbol| DNA::get_rna_symbol(symbol))
            .collect::<String>();

        RNA::new(&rna_string)
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
            .map(|symbol| self.count_symbol(*symbol))
            .collect::<Vec<_>>()
    }

    /// Compute the Hamming Distance between 2 DNA strands
    ///
    /// # Example
    ///
    /// ```
    /// use dna::*;
    /// let dna = DNA::new("ACGT");
    /// let dna2 = DNA::new("ACCC");
    /// assert_eq!(dna.hamm(dna2), 2);
    /// ```
    pub fn hamm(&self, other: &DNA) -> usize {
        let DNA(ref dna_string) = *self;
        let DNA(ref other_dna_string) = *other;
        dna_string.chars().zip(other_dna_string.chars()).filter(|pair| pair.0 != pair.1).count()
    }
}

/// Represents a single strand of RNA
#[derive(Debug, PartialEq)]
pub struct RNA(String);

impl fmt::Display for RNA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let RNA(ref rna_string) = *self;
        write!(f, "{}", rna_string)
    }
}

impl RNA {
    /// Create a new RNA strand
    ///
    /// # Example
    ///
    /// ```
    /// use dna::*;
    ///
    /// let rna = RNA::new("ACGT");
    /// assert_eq!(rna, RNA::new("ACGT"));
    /// ```
    pub fn new(rna_string: &str) -> RNA {
        RNA(rna_string.trim().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_counts_individual_symbols() {
        let dna = DNA::new("AACGGGTTTT");
        assert_eq!(dna.count_symbol('A'), 2);
        assert_eq!(dna.count_symbol('C'), 1);
        assert_eq!(dna.count_symbol('G'), 3);
        assert_eq!(dna.count_symbol('T'), 4);
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
