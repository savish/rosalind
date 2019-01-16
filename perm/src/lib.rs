//! Permutations of vectors

use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

fn generate_lehmer_code(from: i64, pad: usize) -> Vec<i64> {
    let mut remainders: Vec<i64> = vec![];
    let mut quot = from;
    let mut current_digit = 1i64;

    while quot != 0i64 {
        remainders.push(quot % current_digit);
        quot = quot / current_digit;
        current_digit = current_digit + 1;
    }

    let remainders_len = if remainders.len() < pad {
        remainders.len()
    } else {
        pad
    };
    for _ in 0..(pad - remainders_len) {
        remainders.push(0i64);
    }

    remainders.into_iter().rev().collect::<Vec<i64>>()
}

fn replace_zeros(input: i64) -> i64 {
    if input == 0 {
        -1
    } else {
        input
    }
}

pub fn generate_binary(from: u64, pad_to: usize) -> Vec<i64> {
    format!("{:0pad$b}", from, pad = pad_to)
        .chars()
        .map(|ch| replace_zeros(i64::from_str(&ch.to_string()).unwrap()))
        .collect::<Vec<_>>()
}

/// Generate the factorial of a given, positive number
///
/// # Example
///
/// ```
/// use perm::*;
///
/// assert_eq!(factorial(5u64), 120u64);
/// ```
pub fn factorial(num: u64) -> u64 {
    if num == 0 || num == 1 {
        return 1u64;
    } else {
        return num * factorial(num - 1);
    }
}

/// Represents a step in an iteration of permutations of a given vector
#[derive(Debug)]
pub struct Permutation {
    curr: usize,
    base_vector: Vec<i64>,
}

/// Wraps a vector to allow for pretty-printing it
pub struct VecWrapper(Vec<i64>);

impl Deref for VecWrapper {
    type Target = Vec<i64>;

    fn deref(&self) -> &Vec<i64> {
        let VecWrapper(ref vec) = *self;
        vec
    }
}

impl fmt::Display for VecWrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let VecWrapper(ref vec) = *self;
        let perm_string = vec.iter().map(|x| x.to_string() + " ").collect::<String>();
        write!(f, "{}", perm_string.trim())
    }
}

impl VecWrapper {
    pub fn new(vec: Vec<i64>) -> VecWrapper {
        VecWrapper(vec)
    }
}

impl Iterator for Permutation {
    type Item = VecWrapper;

    fn next(&mut self) -> Option<VecWrapper> {
        let vector_length = self.base_vector.len();

        if self.curr < factorial(vector_length as u64) as usize {
            let lehmer_code = generate_lehmer_code(self.curr as i64, vector_length);
            let mut _base_vector = self.base_vector.to_vec();
            let perm = lehmer_code
                .iter()
                .map(|i| _base_vector.remove((*i) as usize))
                .collect::<Vec<_>>();

            self.curr = self.curr + 1;
            Some(VecWrapper::new(perm))
        } else {
            None
        }
    }
}

/// Iterate through the permutations of a given vector
pub fn permutations(vector: Vec<i64>) -> Permutation {
    Permutation {
        curr: 0usize,
        base_vector: vector,
    }
}
