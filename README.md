# Pancake Stack

[![crates.io](https://img.shields.io/crates/v/pancakestack.svg)](https://crates.io/crates/pancakestack)
[![Documentation](https://docs.rs/pancakestack/badge.svg)](https://docs.rs/pancakestack)
[![MIT](https://img.shields.io/crates/l/pancakestack.svg)](./LICENSE)

This is a Rust implementation of the [Pancake Stack](https://esolangs.org/wiki/Pancake_Stack) esoteric programming language. This crate includes a parser and an interpreter.

> Pancake Stack is a stack-based esoteric programming language created by User [JWinslow23](https://esolangs.org/wiki/User:JWinslow23) in [2013](https://en.wikipedia.org/wiki/2013), in which programs require you to manipulate a stack of [pancakes](https://i.ytimg.com/vi/FLd00Bx4tOk/maxresdefault.jpg).

![](https://i.ytimg.com/vi/FLd00Bx4tOk/maxresdefault.jpg)

## Usage

To use pancakestack, first include this in your Cargo.toml:
```toml
[dependencies]
"pancakestack" = "0.2"
```

## Crate Examples

**Basic Usage**

A program can be parsed with [`pancakestack::parse_program_str`](https://docs.rs/pancakestack/*/pancakestack/parse/fn.parse_program_str.html) and run with [`pancakestack::run_program`](https://docs.rs/pancakestack/*/pancakestack/interpret/fn.run_program.html).

```rust
// load program from file
let mut file = File::open("example.pancake").unwrap();
let mut program_str = String::new();
file.read_to_string(&mut program_str).unwrap();

// parse the program
let program = pancakestack::parse_program_str(&program_str);

// run the program
pancakestack::run_program(&program, std::io::stdin(), std::io::stdout()).unwrap();
```

Alternatively you can run a program from a [str](https://doc.rust-lang.org/std/primitive.str.html) or a [Read](https://doc.rust-lang.org/std/io/trait.Read.html) with [`pancakestack::run_program_str`](https://docs.rs/pancakestack/*/pancakestack/interpret/fn.run_program_str.html) and [`pancakestack::run_program_from_read`](https://docs.rs/pancakestack/*/pancakestack/interpret/fn.run_program_from_read.html) respectively.

```rust
// load script file
let mut file = File::open("example.pancake").unwrap();

// write program into string
let mut program = String::new();
file.read_to_string(&mut program).unwrap();

pancakestack::run_program_str(&program, std::io::stdin(), std::io::stdout()).unwrap();
```

```rust
// open script file
let mut file = File::open("example.pancake").unwrap();

// run the script directly from the file
pancakestack::run_program_from_read(file, std::io::stdin(), std::io::stdout()).unwrap();
```

All `pancakestack::run_*`methods accept a [`Read`](https://doc.rust-lang.org/std/io/trait.Read.html) as the input of the script and a [`Write`](https://doc.rust-lang.org/std/io/trait.Write.html) as the output.

The examples until now used [`stdin()`](https://doc.rust-lang.org/std/io/fn.stdin.html) and [`stdout()`](https://doc.rust-lang.org/std/io/fn.stdout.html), but it is possible to use anything implementing [`Read`](https://doc.rust-lang.org/std/io/trait.Read.html) and [`Write`](https://doc.rust-lang.org/std/io/trait.Write.html) respectively. The following example shows the use of strings as input and output:

```rust
let file = File::open("example.pancake").unwrap();
let input = b"some input";
let mut output_buf = Vec::new();
pancakestack::run_program_from_read(file, &input[..], &mut output_buf).unwrap();
let output = std::str::from_utf8(&output_buf).unwrap();
```


**Construct programs**

A program can be parsed from a [`str`](https://doc.rust-lang.org/std/str/) with [`pancakestack::run_program_str`](https://docs.rs/pancakestack/*/pancakestack/interpret/fn.run_program_str.html). A single line (=command) can be parsed with [`BorrowedCommand::from_line`](https://docs.rs/pancakestack/*/pancakestack/parse/enum.BorrowedCommand.html#method.from_line).

Parsed programs are slices of [`BorrowedCommand`](https://docs.rs/pancakestack/*/pancakestack/parse/enum.BorrowedCommand.html)s and can be run with [`pancakestack::run_program`](https://docs.rs/pancakestack/*/pancakestack/interpret/fn.run_program.html).

```rust
use pancakestack::BorrowedCommand;

let program = [
    BorrowedCommand::PutThisPancakeOnTop("test"),
    BorrowedCommand::ShowMeAPancake,
    BorrowedCommand::EatAllOfThePancakes
];
pancakestack::run_program(&program, std::io::stdin(), std::io::stdout()).unwrap();
```


## Language Syntax

The pancake stack starts out as empty.

| Code | Meaning |
| ---- | ------- |
| Put this X pancake on top! | Push the word length of X on top of the stack, i.e. "wonderful" would push 9. |
| Eat the pancake on top! | Pop the top value off of the stack, and discard it. |
| Put the top pancakes together! | Pop off the top two values, add them, and push the result. |
| Give me a pancake! | Input a number value and push it on the stack. |
| How about a hotcake? | Input an ASCII value and push it on the stack. |
| Show me a pancake! | Output the top value on the stack as an ASCII character, but don't pop it. |
| Take from the top pancakes! | Pop off the top two values, subtract the second one from the first one, and push the result. |
| Flip the pancakes on top! | Pop off the top two values, swap them, and push them back. |
| Put another pancake on top! | Pop off the top value and push it twice. |
| \[label\] | Defines a label to go back to (Can also define a comment, if needed). When you go back to the label, it goes to the line number (1 indexed) of the top value of the stack when the label was defined. |
| If the pancake isn't tasty, go over to "label". | Go to label \[label\] if the top value is 0. |
| If the pancake is tasty, go over to "label". | Same as above, except go if the top value is not 0. |
| Put syrup on the pancakes! | Increment all stack values. |
| Put butter on the pancakes! | Increment only the top stack value. |
| Take off the syrup! | Decrement all stack values. |
| Take off the butter! | Decrement only the top stack value. |
| Eat all of the pancakes! | Terminate the program. |

**Implementation Notes:**
- `How about a hotcake?` pushes `0` when there is no input left.
- `[label]` overrides an existing label with the same name.
- Over- and underflowing `u32` will lead to an error (not a `panic`).

## Language Examples

### Hello World!
```pancake
Put this heavenly pancake on top!
Put another pancake on top!
Put another pancake on top!
Put another pancake on top!
Put another pancake on top!
Put another pancake on top!
Put another pancake on top!
Put another pancake on top!
Put syrup on the pancakes!
Put the top pancakes together!
Put the top pancakes together!
Put the top pancakes together!
Put the top pancakes together!
Put the top pancakes together!
Put the top pancakes together!
Put the top pancakes together!
Show me a pancake!
Put this appetizing pancake on top!
Put this delectable pancake on top!
Put this delicious pancake on top!
Put the top pancakes together!
Put the top pancakes together!
Put the top pancakes together!
Show me a pancake!
Put this wonderful pancake on top!
Take off the syrup!
Put the top pancakes together!
Show me a pancake!
Show me a pancake!
Put this rich pancake on top!
Take off the butter!
Put the top pancakes together!
Show me a pancake!
Put this delightful pancake on top!
Put this dainty pancake on top!
Put the top pancakes together!
Put another pancake on top!
Put the top pancakes together!
Show me a pancake!
Put this tasty pancake on top!
Put another pancake on top!
Put another pancake on top!
Put another pancake on top!
Put another pancake on top!
Put another pancake on top!
Put another pancake on top!
Put another pancake on top!
Put another pancake on top!
Put another pancake on top!
Put another pancake on top!
Put the top pancakes together!
Put the top pancakes together!
Put the top pancakes together!
Put the top pancakes together!
Put the top pancakes together!
Put the top pancakes together!
Put the top pancakes together!
Put the top pancakes together!
Put the top pancakes together!
Put the top pancakes together!
Put the top pancakes together!
Show me a pancake!
Eat the pancake on top!
Show me a pancake!
Put this good pancake on top!
Take off the butter!
Put the top pancakes together!
Show me a pancake!
Put this divine pancake on top!
Flip the pancakes on top!
Take from the top pancakes!
Show me a pancake!
Put this pleasant pancake on top!
Flip the pancakes on top!
Take from the top pancakes!
Show me a pancake!
Put this mouthwatering pancake on top!
Put this scrumptious pancake on top!
Put this enjoyable pancake on top!
Put the top pancakes together!
Put the top pancakes together!
Show me a pancake!
Eat all of the pancakes!
```

### Cat
```pancake
Put this old pancake on top!
[CAT]
Eat the pancake on top!
How about a hotcake?
Show me a pancake!
If the pancake is tasty, go over to "CAT".
Eat all of the pancakes!
```

Other examples can be found in the [examples](https://github.com/OpenByteDev/pancakestack/tree/master/examples) directory.

## License
Licensed under MIT license ([LICENSE](./LICENSE) or http://opensource.org/licenses/MIT)
