use crate::command::*;
use std::collections::HashMap;
use std::fmt::{self, Display};
use std::io::{self, prelude::*, BufReader, Read, Write};
use std::*;
use unicode_segmentation::UnicodeSegmentation;

pub fn run_program_from_read<P, I, O>(program: P, input: I, mut output: O) -> Result<(), Error>
where
    P: Read,
    I: Read,
    O: Write,
{
    let mut program = BufReader::new(program);
    let mut input = BufReader::new(input);

    let mut stack = Vec::new();
    let mut labels = HashMap::new();
    let mut executed = Vec::new();
    let mut current_statement: Option<usize> = None;

    let mut program_line = String::new();
    let mut in_line = String::new();
    loop {
        let command = match current_statement {
            Some(ref mut index) => match executed.get(*index) {
                Some(c) => {
                    *index += 1;
                    c
                }
                None => {
                    current_statement = None;
                    continue;
                }
            },
            None => {
                program_line.clear();
                let length = program.read_line(&mut program_line)?;
                if length == 0 {
                    return Ok(());
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

                let c = OwnedCommand::from_line(&program_line);
                if c.is_err() {
                    if !program_line.trim().is_empty() {
                        eprintln!("invalid command: \"{}\"", program_line);
                    }
                    continue;
                }
                let c = c.unwrap().to_owned();
                executed.push(c.clone());
                executed.last().unwrap()
            }
        };

        // println!("{:?} {:?}", stack, command);
        
        match command {
            OwnedCommand::PutThisPancakeOnTop(adjective) => {
                stack.push(adjective.graphemes(true).count() as u32);
            }
            OwnedCommand::EatThePancakeOnTop => {
                if stack.is_empty() {
                    return Err(Error::OutOfPancakes);
                }
                stack.pop();
            }
            OwnedCommand::PutTheTopPancakesTogether => {
                if stack.len() < 2 {
                    return Err(Error::OutOfPancakes);
                }
                let first = stack.pop().unwrap();
                let second = stack.pop().unwrap();
                let result = first.checked_add(second).ok_or(Error::PancakeOverflow)?;
                stack.push(result);
            }
            OwnedCommand::GiveMeAPancake => {
                input.read_line(&mut in_line)?;
                let number_input = in_line
                    .parse()
                    .or_else(|_| Err(Error::InvalidPancake(in_line.clone())))?;
                stack.push(number_input);
                in_line.clear();
            }
            OwnedCommand::HowAboutAHotcake => {
                let buf = input.fill_buf()?;
                let number_input = 
                    if buf.len() == 0 {
                        0
                    } else {
                        buf[0]
                    };
                input.consume(1);
                stack.push(number_input as u32);
            }
            OwnedCommand::ShowMeAPancake => {
                if stack.is_empty() {
                    return Err(Error::OutOfPancakes);
                }
                let top = stack.last().unwrap();
                let c = char::from_u32(*top).ok_or(Error::CanNotShowPancake(*top))?;
                write!(output, "{}", c)?;
            }
            OwnedCommand::TakeFromTheTopPancakes => {
                if stack.len() < 2 {
                    return Err(Error::OutOfPancakes);
                }
                let first = stack.pop().unwrap();
                let second = stack.pop().unwrap();
                let result = first.checked_sub(second).ok_or(Error::PancakeUnderflow)?;
                stack.push(result);
            }
            OwnedCommand::FlipThePancakesOnTop => {
                if stack.len() < 2 {
                    return Err(Error::OutOfPancakes);
                }
                let first = stack.pop().unwrap();
                let second = stack.pop().unwrap();
                stack.push(first);
                stack.push(second);
            }
            OwnedCommand::PutAnotherPancakeOnTop => {
                if stack.is_empty() {
                    return Err(Error::OutOfPancakes);
                }
                stack.push(stack.last().unwrap().clone());
            }
            OwnedCommand::Label(label) => {
                if stack.is_empty() {
                    return Err(Error::OutOfPancakes);
                }
                let top = stack.last().unwrap();
                labels.insert(label.clone(), (*top-1) as usize);
            }
            OwnedCommand::IfThePancakeIsntTastyGoOverTo(target_label) => {
                if stack.is_empty() {
                    return Err(Error::OutOfPancakes);
                }
                let top = *stack.last().unwrap();
                if top == 0 {
                    let label_position = labels
                        .get(target_label)
                        .ok_or(Error::UndefinedLabel(target_label.clone()))?;
                    current_statement = Some(*label_position);
                }
            }
            OwnedCommand::IfThePancakeIsTastyGoOverTo(target_label) => {
                if stack.is_empty() {
                    return Err(Error::OutOfPancakes);
                }
                let top = *stack.last().unwrap();
                if top != 0 {
                    let label_position = labels
                        .get(target_label)
                        .ok_or(Error::UndefinedLabel(target_label.clone()))?;
                    current_statement = Some(*label_position);
                }
            }
            OwnedCommand::PutSyrupOnThePancakes => {
                for value in stack.iter_mut() {
                    *value = value.checked_add(1).ok_or(Error::PancakeOverflow)?;
                }
            }
            OwnedCommand::PutButterOnThePancakes => {
                if stack.is_empty() {
                    return Err(Error::OutOfPancakes);
                }
                let top = stack.last_mut().unwrap();
                *top = top.checked_add(1).ok_or(Error::PancakeOverflow)?;
            }
            OwnedCommand::TakeOffTheSyrup => {
                for value in stack.iter_mut() {
                    *value = value.checked_sub(1).ok_or(Error::PancakeUnderflow)?;
                }
            }
            OwnedCommand::TakeOffTheButter => {
                if stack.is_empty() {
                    return Err(Error::OutOfPancakes);
                }
                let top = stack.last_mut().unwrap();
                *top = top.checked_sub(1).ok_or(Error::PancakeUnderflow)?;
            }
            OwnedCommand::EatAllOfThePancakes => {
                break;
            }
        }
    }
    Ok(())
}
pub fn run_program_str<I, O>(program: &str, input: I, output: O) -> Result<(), Error>
where
    I: Read,
    O: Write,
{
    let parsed = parse_program_str(program);
    run_program(&parsed, input, output)
}
pub fn run_program<I, O>(
    program: &Vec<BorrowedCommand<'_>>,
    input: I,
    mut output: O,
) -> Result<(), Error>
where
    I: Read,
    O: Write,
{
    let mut input = BufReader::new(input);

    let mut stack = Vec::new();
    let mut labels = HashMap::new();
    let mut current_statement: usize = 0;
    let mut in_line = String::new();
    loop {
        let command = match program.get(current_statement) {
            Some(c) => c,
            None => break,
        };
        current_statement += 1;

        // println!("{:?}", command);

        match command {
            BorrowedCommand::PutThisPancakeOnTop(adjective) => {
                stack.push(adjective.graphemes(true).count() as u32);
            }
            BorrowedCommand::EatThePancakeOnTop => {
                if stack.is_empty() {
                    return Err(Error::OutOfPancakes);
                }
                stack.pop();
            }
            BorrowedCommand::PutTheTopPancakesTogether => {
                if stack.len() < 2 {
                    return Err(Error::OutOfPancakes);
                }
                let first = stack.pop().unwrap();
                let second = stack.pop().unwrap();
                let result = first.checked_add(second).ok_or(Error::PancakeOverflow)?;
                stack.push(result);
            }
            BorrowedCommand::GiveMeAPancake => {
                input.read_line(&mut in_line)?;
                let number_input = in_line
                    .parse()
                    .or_else(|_| Err(Error::InvalidPancake(in_line.clone())))?;
                stack.push(number_input);
                in_line.clear();
            }
            BorrowedCommand::HowAboutAHotcake => {
                let buf = input.fill_buf()?;
                let number_input = 
                    if buf.len() == 0 {
                        0
                    } else {
                        buf[0]
                    };
                input.consume(1);
                stack.push(number_input as u32);
            }
            BorrowedCommand::ShowMeAPancake => {
                if stack.is_empty() {
                    return Err(Error::OutOfPancakes);
                }
                let top = stack.last().unwrap();
                let c = char::from_u32(*top).ok_or(Error::CanNotShowPancake(*top))?;
                write!(output, "{}", c)?;
            }
            BorrowedCommand::TakeFromTheTopPancakes => {
                if stack.len() < 2 {
                    return Err(Error::OutOfPancakes);
                }
                let first = stack.pop().unwrap();
                let second = stack.pop().unwrap();
                let result = first.checked_sub(second).ok_or(Error::PancakeUnderflow)?;
                stack.push(result);
            }
            BorrowedCommand::FlipThePancakesOnTop => {
                if stack.len() < 2 {
                    return Err(Error::OutOfPancakes);
                }
                let first = stack.pop().unwrap();
                let second = stack.pop().unwrap();
                stack.push(first);
                stack.push(second);
            }
            BorrowedCommand::PutAnotherPancakeOnTop => {
                if stack.is_empty() {
                    return Err(Error::OutOfPancakes);
                }
                stack.push(stack.last().unwrap().clone());
            }
            BorrowedCommand::Label(label) => {
                if stack.is_empty() {
                    return Err(Error::OutOfPancakes);
                }
                let top = stack.last().unwrap();
                labels.insert(label.clone(), (*top-1) as usize);
            }
            BorrowedCommand::IfThePancakeIsntTastyGoOverTo(target_label) => {
                if stack.is_empty() {
                    return Err(Error::OutOfPancakes);
                }
                let top = *stack.last().unwrap();
                if top == 0 {
                    let label_position = labels
                        .get(target_label)
                        .ok_or(Error::UndefinedLabel(target_label.to_string()))?;
                    current_statement = *label_position;
                }
            }
            BorrowedCommand::IfThePancakeIsTastyGoOverTo(target_label) => {
                if stack.is_empty() {
                    return Err(Error::OutOfPancakes);
                }
                let top = *stack.last().unwrap();
                if top != 0 {
                    let label_position = labels
                        .get(target_label)
                        .ok_or(Error::UndefinedLabel(target_label.to_string()))?;
                    current_statement = *label_position;
                }
            }
            BorrowedCommand::PutSyrupOnThePancakes => {
                for value in stack.iter_mut() {
                    *value = value.checked_add(1).ok_or(Error::PancakeOverflow)?;
                }
            }
            BorrowedCommand::PutButterOnThePancakes => {
                if stack.is_empty() {
                    return Err(Error::OutOfPancakes);
                }
                let top = stack.last_mut().unwrap();
                *top = top.checked_add(1).ok_or(Error::PancakeOverflow)?;
            }
            BorrowedCommand::TakeOffTheSyrup => {
                for value in stack.iter_mut() {
                    *value = value.checked_sub(1).ok_or(Error::PancakeUnderflow)?;
                }
            }
            BorrowedCommand::TakeOffTheButter => {
                if stack.is_empty() {
                    return Err(Error::OutOfPancakes);
                }
                let top = stack.last_mut().unwrap();
                *top = top.checked_sub(1).ok_or(Error::PancakeUnderflow)?;
            }
            BorrowedCommand::EatAllOfThePancakes => {
                break;
            }
        }
    }
    Ok(())
}

pub fn parse_program_str<'a>(program: &'a str) -> Vec<BorrowedCommand<'a>> {
    program
        .lines()
        .filter_map(|line| BorrowedCommand::from_line(line).ok())
        .collect()
}

#[derive(Debug)]
pub enum Error {
    OutOfPancakes,
    InvalidPancake(String),
    CanNotShowPancake(u32),
    UndefinedLabel(String),
    PancakeUnderflow,
    PancakeOverflow,
    Io(io::Error),
}
impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::OutOfPancakes => write!(f, "Out of pancakes!"),
            Error::InvalidPancake(s) => write!(f, "Invalid pancake: {}", s),
            Error::CanNotShowPancake(p) => {
                write!(f, "Pancake can not be shown (invalid char): {}", p)
            }
            Error::UndefinedLabel(l) => write!(f, "Use of undefined label \"{}\"", l),
            Error::PancakeUnderflow => write!(f, "Pancake underflowed its domain."),
            Error::PancakeOverflow => write!(f, "Pancake overflowed its domain."),
            Error::Io(io) => io.fmt(f),
        }
    }
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}
