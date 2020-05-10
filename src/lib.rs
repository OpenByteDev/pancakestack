#[macro_use]
extern crate lazy_static;

use std::char;
use std::str;
use regex::Regex;
use std::io::{Read, BufReader, Write};
use std::collections::{HashMap};
use unicode_segmentation::UnicodeSegmentation;
use std::io::prelude::*;

// https://esolangs.org/wiki/Pancake_Stack

#[derive(Debug, Clone)]
/*enum Command<'a> {
    PutThisPancakeOnTop(&'a str),
    EatThePancakeOnTop,
    PutTheTopPancakesTogether,
    GiveMeAPancake,
    HowAboutAHotcake,
    ShowMeAPancake,
    TakeFromTheTopPancakes,
    FlipThePancakesOnTop,
    PutAnotherPancakeOnTop,
    Label(&'a str),
    IfThePancakeIsntTastyGoOverTo(&'a str),
    IfThePancakeIsTastyGoOverTo(&'a str),
    PutSyrupOnThePancakes,
    PutButterOnThePancakes,
    TakeOffTheSyrup,
    TakeOffTheButter,
    EatAllOfThePancakes
}*/
pub enum Command {
    PutThisPancakeOnTop(String),
    EatThePancakeOnTop,
    PutTheTopPancakesTogether,
    GiveMeAPancake,
    HowAboutAHotcake,
    ShowMeAPancake,
    TakeFromTheTopPancakes,
    FlipThePancakesOnTop,
    PutAnotherPancakeOnTop,
    Label(String),
    IfThePancakeIsntTastyGoOverTo(String),
    IfThePancakeIsTastyGoOverTo(String),
    PutSyrupOnThePancakes,
    PutButterOnThePancakes,
    TakeOffTheSyrup,
    TakeOffTheButter,
    EatAllOfThePancakes
}

pub struct PancakeStackParser {
}
impl PancakeStackParser {
    pub fn parse_line(line: &str) -> Option<Command> {
        lazy_static! {
            static ref PUT_THIS_PANCAKE_ON_TOP_REGEX: Regex = Regex::new(r"^Put this (\S*) pancake on top!$").unwrap();
            static ref EAT_THE_PANCAKE_ON_TOP_REGEX: Regex = Regex::new(r"^Eat the pancake on top!$").unwrap();
            static ref PUT_THE_TOP_PANCAKES_TOGETHER_REGEX: Regex = Regex::new(r"^Put the top pancakes together!$").unwrap();
            static ref GIVE_ME_A_PANCAKE_REGEX: Regex = Regex::new(r"^Give me a pancake!$").unwrap();
            static ref HOW_ABOUT_A_HOTCAKE_REGEX: Regex = Regex::new(r"^How about a hotcake\?$").unwrap();
            static ref SHOW_ME_A_PANCAKE_REGEX: Regex = Regex::new(r"^Show me a pancake!$").unwrap();
            static ref TAKE_FROM_THE_TOP_PANCAKES_REGEX: Regex = Regex::new(r"^Take from the top pancakes!$").unwrap();
            static ref FLIP_THE_PANCAKES_ON_TOP_REGEX: Regex = Regex::new(r"^Flip the pancakes on top!$").unwrap();
            static ref PUT_ANOTHER_PANCAKE_ON_TOP_REGEX: Regex = Regex::new(r"^Put another pancake on top!$").unwrap();
            static ref LABEL_REGEX: Regex = Regex::new(r"^\[(.*)\]$").unwrap();
            static ref IF_THE_PANCAKE_ISNT_TASTY_GO_OVER_TO_REGEX: Regex = Regex::new("^If the pancake isn't tasty, go over to \"(.*)\".$").unwrap();
            static ref IF_THE_PANCAKE_IS_TASTY_GO_OVER_TO_REGEX: Regex = Regex::new("^If the pancake is tasty, go over to \"(.*)\".$").unwrap();
            static ref PUT_SYRUP_ON_THE_PANCAKES_REGEX: Regex = Regex::new(r"^Put syrup on the pancakes!$").unwrap();
            static ref PUT_BUTTER_ON_THE_PANCAKES_REGEX: Regex = Regex::new(r"^Put butter on the pancakes$").unwrap();
            static ref TAKE_OFF_THE_SYRUP_REGEX: Regex = Regex::new(r"^Take off the syrup!$").unwrap();
            static ref TAKE_OFF_THE_BUTTER_REGEX: Regex = Regex::new(r"^Take off the butter!$").unwrap();
            static ref EAT_ALL_OF_THE_PANCAKES_REGEX: Regex = Regex::new(r"^Eat all of the pancakes!$").unwrap();
        }
        
        if let Some(captures) = PUT_THIS_PANCAKE_ON_TOP_REGEX.captures_iter(line).next() {
            return Some(Command::PutThisPancakeOnTop(captures[1].to_string()));
            // return Some(Command::PutThisPancakeOnTop(captures.get(1).unwrap().as_str()));
        }
        if EAT_THE_PANCAKE_ON_TOP_REGEX.is_match(line) {
            return Some(Command::EatThePancakeOnTop);
        }
        if PUT_THE_TOP_PANCAKES_TOGETHER_REGEX.is_match(line) {
            return Some(Command::PutTheTopPancakesTogether);
        }
        if GIVE_ME_A_PANCAKE_REGEX.is_match(line) {
            return Some(Command::GiveMeAPancake);
        }
        if HOW_ABOUT_A_HOTCAKE_REGEX.is_match(line) {
            return Some(Command::HowAboutAHotcake);
        }
        if SHOW_ME_A_PANCAKE_REGEX.is_match(line) {
            return Some(Command::ShowMeAPancake);
        }
        if TAKE_FROM_THE_TOP_PANCAKES_REGEX.is_match(line) {
            return Some(Command::TakeFromTheTopPancakes);
        }
        if FLIP_THE_PANCAKES_ON_TOP_REGEX.is_match(line) {
            return Some(Command::FlipThePancakesOnTop);
        }
        if PUT_ANOTHER_PANCAKE_ON_TOP_REGEX.is_match(line) {
            return Some(Command::PutAnotherPancakeOnTop);
        }
        if let Some(captures) = LABEL_REGEX.captures_iter(line).next() {
            return Some(Command::Label(captures[1].to_string()));
            // return Some(Command::Label(captures.get(1).unwrap().as_str()));
        }
        if let Some(captures) = IF_THE_PANCAKE_ISNT_TASTY_GO_OVER_TO_REGEX.captures_iter(line).next() {
            return Some(Command::IfThePancakeIsntTastyGoOverTo(captures[1].to_string()));
            // return Some(Command::IfThePancakeIsntTastyGoOverTo(captures.get(1).unwrap().as_str()));
        }
        if let Some(captures) = IF_THE_PANCAKE_IS_TASTY_GO_OVER_TO_REGEX.captures_iter(line).next() {
            return Some(Command::IfThePancakeIsTastyGoOverTo(captures[1].to_string()));
            //return Some(Command::IfThePancakeIsTastyGoOverTo(captures.get(1).unwrap().as_str()));
        }
        if PUT_SYRUP_ON_THE_PANCAKES_REGEX.is_match(line) {
            return Some(Command::PutSyrupOnThePancakes)
        }
        if PUT_BUTTER_ON_THE_PANCAKES_REGEX.is_match(line) {
            return Some(Command::PutButterOnThePancakes)
        }
        if TAKE_OFF_THE_SYRUP_REGEX.is_match(line) {
            return Some(Command::TakeOffTheSyrup)
        }
        if TAKE_OFF_THE_BUTTER_REGEX.is_match(line) {
            return Some(Command::TakeOffTheButter)
        }
        if EAT_ALL_OF_THE_PANCAKES_REGEX.is_match(line) {
            return Some(Command::EatAllOfThePancakes)
        }
        None
    }

}

pub struct PancakeStack {
}
impl PancakeStack {
    pub fn new() -> Self {
        PancakeStack { }
    }
    pub fn run_program<P, I, O: Write>(&mut self, program_read: P, input_read: I, output: &mut O) where P:Read, I:Read, O:Write {
        let mut stack = Vec::new();
        let mut labels = HashMap::new();
        let mut program = Vec::new();
        let mut current_statement: Option<usize> = None;

        let mut program_reader = BufReader::new(program_read);
        let mut input_reader = BufReader::new(input_read);

        let mut program_line = String::new();
        let mut input_line = String::new();
        'outer: loop {
            let command = match current_statement {
                Some(ref mut index) => {
                    match program.get(*index) {
                        Some(c) => {
                            *index += 1;
                            c
                        },
                        None => {
                            current_statement = None;
                            continue;
                        }
                    }
                },
                None => {
                    program_line.clear();
                    let length = program_reader.read_line(&mut program_line).unwrap();
                    if length == 0 {
                        break 'outer;
                    }
                    trim_newline(&mut program_line);
                    fn trim_newline(s: &mut String) {
                        if s.ends_with('\n') {
                            s.pop();
                            if s.ends_with('\r') {
                                s.pop();
                            }
                        }
                    }

                    let c = PancakeStackParser::parse_line(&program_line);
                    if c.is_none() {
                        eprintln!("{:?}", &program_line);
                        continue;
                    }
                    let c = c.unwrap();
                    program.push(c.clone());
                    program.last().unwrap()
                }
            };

            println!("{:?}", command);
            
            match command {
                Command::PutThisPancakeOnTop(adjective) => {
                    stack.push(adjective.graphemes(true).count() as u32);
                },
                Command::EatThePancakeOnTop => {
                    stack.pop();
                },
                Command::PutTheTopPancakesTogether => {
                    let first = stack.pop().unwrap();
                    let second = stack.pop().unwrap();
                    stack.push(first + second);
                },
                Command::GiveMeAPancake => {
                    input_reader.read_line(&mut input_line).unwrap();
                    let number_input = input_line.parse().unwrap();
                    stack.push(number_input);
                    input_line.clear();
                },
                Command::HowAboutAHotcake => {
                    let buf = input_reader.fill_buf().unwrap();
                    if buf.len() == 0 {
                        // TODO define behaviour
                        stack.push(0);
                        continue;
                    }
                    let number_input = buf[0];
                    input_reader.consume(1);
                    stack.push(number_input as u32);
                },
                Command::ShowMeAPancake => {
                    let top = stack.last().unwrap();
                    let c = char::from_u32(*top).unwrap();
                    write!(output, "{}", c).unwrap();
                },
                Command::TakeFromTheTopPancakes => {
                    let first = stack.pop().unwrap();
                    let second = stack.pop().unwrap();
                    stack.push(first - second);
                },
                Command::FlipThePancakesOnTop => {
                    let first = stack.pop().unwrap();
                    let second = stack.pop().unwrap();
                    stack.push(first);
                    stack.push(second);
                },
                Command::PutAnotherPancakeOnTop => {
                    stack.push(stack.last().unwrap().clone());
                }
                Command::Label(label) => {
                    labels.insert(label.clone(), program.len()); // save position of label
                },
                Command::IfThePancakeIsntTastyGoOverTo(target_label) => {
                    let top = *stack.last().unwrap();
                    if top == 0 {
                        let label_position = labels.get(target_label).unwrap();
                        current_statement = Some(*label_position);
                    }
                },
                Command::IfThePancakeIsTastyGoOverTo(target_label) => {
                    let top = *stack.last().unwrap();
                    if top != 0 {
                        let label_position = labels.get(target_label).unwrap();
                        current_statement = Some(*label_position);
                    }
                },
                Command::PutSyrupOnThePancakes => {
                    for value in stack.iter_mut() {
                        *value += 1;
                    }
                },
                Command::PutButterOnThePancakes => {
                    let top = stack.last_mut().unwrap();
                    *top += 1;
                },
                Command::TakeOffTheSyrup => {
                    for value in stack.iter_mut() {
                        *value -= 1;
                    }
                },
                Command::TakeOffTheButter => {
                    let top = stack.last_mut().unwrap();
                    *top -= 1;
                },
                Command::EatAllOfThePancakes => {
                    break;
                }
            }
        }
    }
}

