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

    println!("{:?}\n", population(params[1]).nth(params[0] - 1).unwrap());
}

fn description() {
    println!("\nRabbits and Recurrence Relations\n");
    println!("Enter 2 numbers, n and k. The algorithm will determine the total ");
    println!("number of rabbit pairs that will be present after n months, if ");
    println!("we begin with 1 pair and in each generation, every pair of ");
    println!("reproduction-age rabbits produces a litter of k rabbit pairs.\n");
    println!("For instance: 5 3\n");
}
