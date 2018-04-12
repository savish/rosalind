extern crate rosalind;

use rosalind::hamming_distance;
use std::io;

fn main() {
    let mut first_string = String::new();
    io::stdin().read_line(&mut first_string).unwrap();
    let mut second_string = String::new();
    io::stdin().read_line(&mut second_string).unwrap();

    println!("{}", hamming_distance(&first_string, &second_string));
}
