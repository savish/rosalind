extern crate fib;

use fib::*;
use std::io;
use std::str::FromStr;

fn main() {
    description();
    let mut param_string = String::new();
    io::stdin().read_line(&mut param_string).unwrap();
    let params = param_string
        .trim()
        .split(' ')
        .map(|x| usize::from_str(x).unwrap())
        .collect::<Vec<_>>();

    println!(
        "{:?}\n",
        population_with_moratilty(1, params[1])
            .nth(params[0] - 1)
            .unwrap()
    );
}

fn description() {
    println!("\nMortal Fibonacci Rabbits\n");
    println!("Enter 2 numbers, n and m. The algorithm will determine the total ");
    println!("number of rabbit pairs that will remain after the n-th month, if ");
    println!("all rabbits live for m months\n");
    println!("For instance: 6 3\n");
}
