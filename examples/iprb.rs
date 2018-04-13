extern crate gene;

use std::io;
use gene::*;

fn main() {
    let mut pop_counts = String::new();
    io::stdin().read_line(&mut pop_counts).unwrap();

    let pops: Vec<u32> = pop_counts
        .trim()
        .split(" ")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let population = Population::new(pops[0], pops[1], pops[2]);

    // Step 1: generate parent pairs
    let parents = Organism::parents();

    // Step 2: get probabilities of a dominant child from a parent pair
    let dominant_probabilities = parents
        .iter()
        .map(|&(p1, p2)| p1.has_dominant_child(&p2))
        .collect::<Vec<_>>();

    // Step 3: get probabilities of selecting the parent pairs from a population
    let selection_probabilities = parents
        .iter()
        .map(|&(p1, p2)| population.select_parents(&p1, &p2))
        .collect::<Vec<_>>();

    // Step 4: zip, fold
    let result = dominant_probabilities
        .iter()
        .zip(selection_probabilities.iter())
        .fold(0f64, |acc, (p_d, p_s)| acc + p_d * p_s);

    println!("{}", result);
}
