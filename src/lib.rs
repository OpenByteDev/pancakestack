#[macro_use]
extern crate lazy_static;

// https://esolangs.org/wiki/Pancake_Stack

mod command;
mod interpreter;

pub use command::*;
pub use interpreter::*;
