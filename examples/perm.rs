extern crate perm;

use perm::*;
use std::io;
use std::str::FromStr;

fn main() {
    let mut max_permute_string = String::new();
    io::stdin().read_line(&mut max_permute_string).unwrap();
    let permute_to = usize::from_str(&max_permute_string.trim()).unwrap();

    println!("{}", factorial(permute_to as u64));
    for code in permutations((1i64..permute_to as i64 + 1).collect::<Vec<_>>()) {
        println!("{}", code);
    }
}
