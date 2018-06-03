//! # Rosalind project

//! The Rosalind project is a platform for learning bioinformatics through problem solving.
//! More information about the project is available at <http://rosalind.info/about/>
//!
//! This repository contains solutions to the problems listed below.

//! ## 1. Setup
//! This project is written entirely in Rust. Ensure that you are
//! running the latest version of Rust (stable). You can get rust
//! from here <https://www.rust-lang.org>
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

//! ## 2. Usage
//! This repository contains a number of libraries grouped
//! together in a workspace.

//! ### 2.1 Testing
//! From within the `rosalind` repo directory use:
//!
//! ```bash
//! $ cargo test -p <LIBRARY_NAME>
//! ```
//!
//! The available libraries are:
//!
//! - `gen_str`
//! - `fib`
//! - `perm`
//!
//! ### 2.2 Running
//! From within the `rosalind` repo directory use:
//!
//! ```bash
//! $ cargo run --release --example <EXAMPLE_NAME>
//! ```

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
