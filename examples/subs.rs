extern crate rosalind;

use rosalind::substring_locations;
use std::io;

fn main() {
    let mut source_string = String::new();
    io::stdin().read_line(&mut source_string).unwrap();
    let mut substring = String::new();
    io::stdin().read_line(&mut substring).unwrap();

    println!(
        "{:?}",
        substring_locations(&source_string, &substring)
            .iter()
            .map(|x| (x + 1).to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
