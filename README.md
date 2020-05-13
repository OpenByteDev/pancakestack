# Pancake Stack

[![crates.io](https://img.shields.io/crates/v/pancakestack.svg)](https://crates.io/crates/pancakestack)
[![Documentation](https://docs.rs/pancakestack/badge.svg)](https://docs.rs/pancakestack)
[![MIT](https://img.shields.io/crates/l/pancakestack.svg)](./LICENSE)

This is a Rust implementation of the [Pancake Stack](https://esolangs.org/wiki/Pancake_Stack) esoteric programming language. This crate includes a parser and an interpreter.

> Pancake Stack is a stack-based esoteric programming language created by User [JWinslow23](https://esolangs.org/wiki/User:JWinslow23) in [2013](https://en.wikipedia.org/wiki/2013), in which programs require you to manipulate a stack of [pancakes](https://i.ytimg.com/vi/FLd00Bx4tOk/maxresdefault.jpg).

## Usage

To use pancakestack, first add this to your Cargo.toml:
```toml
[dependencies]
"pancakestack" = "0.1.0"
```

## Crate Examples

This examples shows how to run a `.pancake` file using `stdin()` as input and `stdout()` as output.
```rust
let file = File::open("example.pancake").unwrap();
pancakestack::run_program_from_read(file, std::io:stdin(), std::io:stdout()).unwrap();
```

This examples shows how to run a `.pancake` file with strings as input and output.
```rust
let file = File::open("example.pancake").unwrap();
let input = b"some input";
let mut output_buf = Vec::new();
pancakestack::run_program_from_read(file, &input[..], &mut output_buf).unwrap();
let output = str::from_utf8(&output_buf).unwrap();
```

This examples shows how to parse and run a `.pancake` script using `stdin()` as input and `stdout()` as output.
```rust
let file = File::open("example.pancake")?;
let input = b"some input";
let mut output_buf = Vec::new();
pancakestack::run_program_from_read(file, &input[..], &mut output_buf).unwrap();
let output = str::from_utf8(&output_buf).unwrap();

let mut file = File::open("example.pancake").unwrap();
let mut program_str = String::new();
file.read_to_string(&mut program_str).unwrap();

let program = pancakestack::parse_program_str(&program_str);
pancakestack::run_program(&program, std::io:stdin(), std::io:stdout()).unwrap();

```

This examples shows how to construct a script from commands and running it.
```rust
use pancakestack::command::*;

let program = vec![
    BorrowedCommand::PutThisPancakeOnTop("test"),
    BorrowedCommand::ShowMeAPancake,
    BorrowedCommand::EatAllOfThePancakes
];
pancakestack::run_program(&program, std::io:stdin(), std::io:stdout()).unwrap();
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
| [label] | Defines a label to go back to (Can also define a comment, if needed). When you go back to the label, it goes to the line number (1 indexed) of the top value of the stack when the label was defined. |
| If the pancake isn't tasty, go over to "label". | Go to label [label] if the top value is 0. |
| If the pancake is tasty, go over to "label". | Same as above, except go if the top value is not 0. |
| Put syrup on the pancakes! | Increment all stack values. |
| Put butter on the pancakes! | Increment only the top stack value. |
| Take off the syrup! | Decrement all stack values. |
| Take off the butter! | Decrement only the top stack value. |
| Eat all of the pancakes! | Terminate the program. |

**Implementation Notes:**
- `How about a hotcake?` pushes 0 when there is no input left.
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

## License
Licensed under MIT license ([LICENSE](./LICENSE) or http://opensource.org/licenses/MIT)
