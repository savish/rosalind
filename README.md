# Rosalind project

The Rosalind project is a platform for learning bioinformatics through problem solving.
More information about the project is available at http://rosalind.info/about/

This repository contains solutions to some of the rosalind problems. Each propblem is available as a subcommand under the main executable.

## 1. Usage

By default, this application compiles to a binary called `rosalind`. Sample usage is given in the code snippet below:

```bash
# Shows the available subcommands
$ rosalind --help

rosalind 0.1.0
Alan K <afksavish@gmail.com>
Solutions to the Rosalind Project problem set

USAGE:
    rosalind <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    dna     Counting DNA Nucleotides
    fib     Rabbits and Recurrence Relations
    fibd    Mortal Fibonacci Rabbits
    gc      Computing GC Content
    hamm    Counting Point Mutations
    help    Prints this message or the help of the given subcommand(s)
    iprb    Introduction to Mendelian Inheritance
    mrna    Inferring mRNA from Protein
    perm    Enumerating Gene Orders
    prot    Translating RNA into Protein
    revc    Complementing a Strand of DNA
    rna     Transcribing DNA into RNA
    sign    Enumerating Oriented Gene Orderings
    subs    Finding a Motif in DNA
```

### Example subcommand help

```bash
$ rosalind revc --help

rosalind-revc
Complementing a Strand of DNA

USAGE:
    rosalind revc <dna_string>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <dna_string>    A DNA string, upto 1000 nucleotides in length.
```

### Example subcommand

```bash
$ rosalind revc ACGTACGTC
GACGTACGT
```

## 2. Setup

This project is written entirely in Rust (edition 2018).You can get rust
from here https://www.rust-lang.org

Once you have a working installation of rust available,
proceed as follows from a terminal session:

```bash
$ cd <SOME_WORKING_DIR>
$ git clone <THIS_REPO>
$ cd rosalind
$ cargo build
```

## 3 Testing

From within the `rosalind` repo directory use:

```bash
$ cargo test
```
