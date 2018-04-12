extern crate perm;

use std::io;
use std::str::FromStr;
use perm::*;

fn main() {
    let mut max_permute_string = String::new();
    io::stdin().read_line(&mut max_permute_string).unwrap();
    let permute_to = usize::from_str(&max_permute_string.trim()).unwrap();
    let permute_pow2 = 2u64.pow(permute_to as u32);

    // Number of outputs
    println!("{}", factorial(permute_to as u64) * permute_pow2);

    // Permutations
    for code in permutations((1i64..permute_to as i64 + 1).collect::<Vec<_>>()) {
        let vec = &*code; // Deref from wrapper
        for binary in 0..permute_pow2 {
            let binary = generate_binary(binary, permute_to);
            let zipped = binary.iter().zip(vec.iter()).collect::<Vec<_>>();
            let perm = zipped
                .into_iter()
                .map(|val| *val.0 * *val.1 as i64)
                .collect::<Vec<_>>();
            println!("{}", VecWrapper::new(perm));
        }
    }
}
