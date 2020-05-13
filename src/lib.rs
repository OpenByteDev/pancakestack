//! # Pancake Stack
//! This is a Rust implementation of the [Pancake Stack](https://esolangs.org/wiki/Pancake_Stack) esoteric programming language.
//! This crate includes a parser and an interpreter.
//!
//! > Pancake Stack is a stack-based esoteric programming language created by
//! > User [JWinslow23](https://esolangs.org/wiki/User:JWinslow23)
//! > in [2013](https://en.wikipedia.org/wiki/2013), in which programs require you to
//! > manipulate a stack of [pancakes](https://i.ytimg.com/vi/FLd00Bx4tOk/maxresdefault.jpg).
//!
//!
//! **Basic Usage**
//!
//! A program can be parsed with [`pancakestack::parse_program_str`](https://docs.rs/pancakestack/*/pancakestack/fn.parse_program_str.html) and run it with [`pancakestack::run_program`](https://docs.rs/pancakestack/*/pancakestack/fn.run_program.html).
//!
//! ```rust
//! # use std::fs::File;
//! # use std::io::Read;
//! # fn run() {
//! // load program from file
//! let mut file = File::open("example.pancake").unwrap();
//! let mut program_str = String::new();
//! file.read_to_string(&mut program_str).unwrap();
//!
//! // parse the program
//! let program = pancakestack::parse_program_str(&program_str);
//!
//! // run the program
//! pancakestack::run_program(&program, std::io::stdin(), std::io::stdout()).unwrap();
//! # }
//! ```
//!
//! Alternatively you can run a program from a [str](https://doc.rust-lang.org/std/primitive.str.html) or a [Read](https://doc.rust-lang.org/std/io/trait.Read.html) with [`pancakestack::run_program_str`](https://docs.rs/pancakestack/*/pancakestack/fn.run_program_str.html) or [`pancakestack::run_program_from_read`](https://docs.rs/pancakestack/*/pancakestack/fn.run_program_str.html) respectively.
//!
//! ```rust
//! # use std::fs::File;
//! # use std::io::Read;
//! # fn run() {
//! // load script file
//! let mut file = File::open("example.pancake").unwrap();
//!
//! // write program into string
//! let mut program = String::new();
//! file.read_to_string(&mut program).unwrap();
//!
//! pancakestack::run_program_str(&program, std::io::stdin(), std::io::stdout()).unwrap();
//! # }
//! ```
//!
//! ```rust
//! # use std::fs::File;
//! # fn run() {
//! // open script file
//! let mut file = File::open("example.pancake").unwrap();
//!
//! // run the script directly from the file
//! pancakestack::run_program_from_read(file, std::io::stdin(), std::io::stdout()).unwrap();
//! # }
//! ```
//!
//! All `pancakestack::run_*`methods accept and [`Read`](https://doc.rust-lang.org/std/io/trait.Read.html) for the input of the script and a [`Write`](https://doc.rust-lang.org/std/io/trait.Write.html) as the output.
//!
//! The examples until now used [`stdin()`](https://doc.rust-lang.org/std/io/fn.stdin.html) and [`stdout()`](https://doc.rust-lang.org/std/io/fn.stdout.html), but it is possible to use anything implementing [`Read`](https://doc.rust-lang.org/std/io/trait.Read.html) and [`Write`](https://doc.rust-lang.org/std/io/trait.Write.html) respectively. The folowing example shows the use of strings as input and output:
//!
//! ```rust
//! # use std::fs::File;
//! # fn run() {
//! let file = File::open("example.pancake").unwrap();
//! let input = b"some input";
//! let mut output_buf = Vec::new();
//! pancakestack::run_program_from_read(file, &input[..], &mut output_buf).unwrap();
//! let output = std::str::from_utf8(&output_buf).unwrap();
//! # }
//! ```
//!
//!
//! **Construct programs**
//!
//! A program can be parsed from a [`str`](https://doc.rust-lang.org/std/str/) with [`pancakestack::run_program_str`](https://docs.rs/pancakestack/*/pancakestack/fn.run_program_str.html). A single line (=command) can be parsed with [`BorrowedCommand::from_line`](https://docs.rs/pancakestack/*/pancakestack/enum.BorrowedCommand.html#method.from_line).
//!
//! Complete programs can pe constructed by creating a [`Vec`](https://doc.rust-lang.org/std/vec/) of [`BorrowedCommand`](https://docs.rs/pancakestack/*/pancakestack/enum.BorrowedCommand.html)s and run with [`pancakestack::run_program`](https://docs.rs/pancakestack/*/pancakestack/fn.run_program.html).
//!
//! ```rust
//! use pancakestack::BorrowedCommand;
//!
//! let program = vec![
//!     BorrowedCommand::PutThisPancakeOnTop("test"),
//!     BorrowedCommand::ShowMeAPancake,
//!     BorrowedCommand::EatAllOfThePancakes
//! ];
//! pancakestack::run_program(&program, std::io::stdin(), std::io::stdout()).unwrap();
//!

#[macro_use]
extern crate lazy_static;

pub mod command;
pub mod interpreter;

pub use command::*;
pub use interpreter::*;
