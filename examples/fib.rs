extern crate fib;

use fib::*;
use std::io;
use std::str::FromStr;

fn main() {
    let mut param_string = String::new();
    io::stdin().read_line(&mut param_string).unwrap();
    let params = param_string
        .trim()
        .split(' ')
        .map(|x| u64::from_str(x).unwrap())
        .collect::<Vec<_>>();
    println!("{:?}", population(params[1]).nth((params[0] - 1) as usize).unwrap());
}