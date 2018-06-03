# Rosalind project

The Rosalind project is a platform for learning bioinformatics through problem solving.
More information about the project is available at http://rosalind.info/about/

This repository contains solutions to the problems listed below.

## 1. Setup

This project is written entirely in Rust. Ensure that you are
running the latest version of Rust (stable). You can get rust
from here https://www.rust-lang.org

Once you have a working installation of rust available,
proceed as follows from a terminal session:

```bash
$ cd <SOME_WORKING_DIR>
$ git clone <THIS_REPO>
$ cd rosalind
$ cargo build
```

## 2. Usage

This repository contains a number of libraries grouped
together in a workspace.

### 2.1 Testing

From within the `rosalind` repo directory use:

```bash
$ cargo test -p <LIBRARY_NAME>
```

The available libraries are:

* `gen_str`
* `fib`
* `perm`

### 2.2 Running

From within the `rosalind` repo directory use:

```bash
$ cargo run --release --example <EXAMPLE_NAME>
```
