//! # Rosalind project

//! The Rosalind project is a platform for learning bioinformatics through problem solving.
//! More information about the project is available at <http://rosalind.info/about/>
//!
//! This repository contains solutions to the problems listed below.

//! ## 1. Setup
//! This project is written entirely in Rust. Ensure that you are
//! running the latest version of Rust (stable). You can get rust
//! from here <https://www.rust-lang.org>
//!
//! Once you have a working installation of rust available,
//! proceed as follows from a terminal session:
//!
//! ```bash
//! $ cd <SOME_WORKING_DIR>
//! $ git clone <THIS_REPO>
//! $ cd rosalind
//! $ cargo build
//! ```

//! ## 2. Usage
//! This repository contains a number of libraries grouped
//! together in a workspace.

//! ### 2.1 Testing
//! From within the `rosalind` repo directory use:
//!
//! ```bash
//! $ cargo test -p <LIBRARY_NAME>
//! ```
//!
//! The available libraries are:
//!
//! - `gen_str`
//! - `fib`
//! - `perm`
//!
//! ### 2.2 Running
//! From within the `rosalind` repo directory use:
//!
//! ```bash
//! $ cargo run --release --example <EXAMPLE_NAME>
//! ```
//!
//! A list of examples is given below. The example names correspond to the short names assigned to the problems on the Rosalind project website.
//!
//! For convenience, the command used to run the examples is also provided next to the example name.
//!
//! Name | Command | Input
//! --- | --- | ---
//! `dna` | `cargo run --release --example dna` | DNA string
//! `rna` | `cargo run --release --example rna` | DNA string
//! `revc` | `cargo run --release --example revc` | DNA string
//! `fib` | `cargo run --release --example fib` | 2 numbers
//! `fibd` | `cargo run --release --example fibd` | 2 numbers
//! `hamm` | `cargo run --release --example hamm` | 2 strings
//! `perm` | `cargo run --release --example perm` | Number
//! `sign` | `cargo run --release --example sign` | Number
//! `gc` | `cargo run --release --example gc gc.txt` | `gc.txt` at root
//! `prot` | `cargo run --release --example prot prot.txt` | `prot.txt` at root
