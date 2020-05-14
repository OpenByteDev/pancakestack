use regex::Regex;
use std::fmt::{self, Display};

/// Parses the given str into an vec of commands.
/// Each command has to be on its own line.
/// This method does not allocate any strings.
/// ```rust
/// # use std::fs::File;
/// # use std::io::Read;
/// # fn run() {
/// // load program from file
/// let mut file = File::open("example.pancake").unwrap();
/// let mut program_str = String::new();
/// file.read_to_string(&mut program_str).unwrap();
///
/// // parse the program
/// let program = pancakestack::parse_program_str(&program_str);
/// # }
/// ```
pub fn parse_program_str<'a>(program: &'a str) -> Vec<BorrowedCommand<'a>> {
    program
        .lines()
        .filter_map(|line| BorrowedCommand::from_line(line).ok())
        .collect()
}

/// An enum representing a pancakestack command.
/// Labels and pancake adjectives are stored in [`str`](https://doc.rust-lang.org/std/str/)s .
/// See [`OwnedCommand`](./enum.OwnedCommand.html) for a version that uses [`String`](https://doc.rust-lang.org/std/string/struct.String.html)s.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BorrowedCommand<'a> {
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
    EatAllOfThePancakes,
}

impl<'a> BorrowedCommand<'a> {
    /// Parses the given line as a pancake stack command.
    /// The command will reference the strings contents.
    pub fn from_line(line: &'a str) -> Result<Self, ParseCommandError<'a>> {
        lazy_static! {
            static ref PUT_THIS_PANCAKE_ON_TOP_REGEX: Regex =
                Regex::new(r"^Put this (\S*) pancake on top!$").unwrap();
            static ref LABEL_REGEX: Regex = Regex::new(r"^\[(.+)\]$").unwrap();
            static ref IF_THE_PANCAKE_ISNT_TASTY_GO_OVER_TO_REGEX: Regex =
                Regex::new("^If the pancake isn't tasty, go over to \"(.*)\"\\.$").unwrap();
            static ref IF_THE_PANCAKE_IS_TASTY_GO_OVER_TO_REGEX: Regex =
                Regex::new("^If the pancake is tasty, go over to \"(.*)\"\\.$").unwrap();
        }

        match line {
            "Eat the pancake on top!" => Ok(BorrowedCommand::EatThePancakeOnTop),
            "Put the top pancakes together!" => Ok(BorrowedCommand::PutTheTopPancakesTogether),
            "Give me a pancake!" => Ok(BorrowedCommand::GiveMeAPancake),
            "How about a hotcake?" => Ok(BorrowedCommand::HowAboutAHotcake),
            "Show me a pancake!" => Ok(BorrowedCommand::ShowMeAPancake),
            "Take from the top pancakes!" => Ok(BorrowedCommand::TakeFromTheTopPancakes),
            "Flip the pancakes on top!" => Ok(BorrowedCommand::FlipThePancakesOnTop),
            "Put another pancake on top!" => Ok(BorrowedCommand::PutAnotherPancakeOnTop),
            "Put syrup on the pancakes!" => Ok(BorrowedCommand::PutSyrupOnThePancakes),
            "Put butter on the pancakes!" => Ok(BorrowedCommand::PutButterOnThePancakes),
            "Take off the syrup!" => Ok(BorrowedCommand::TakeOffTheSyrup),
            "Take off the butter!" => Ok(BorrowedCommand::TakeOffTheButter),
            "Eat all of the pancakes!" => Ok(BorrowedCommand::EatAllOfThePancakes),
            _ => {
                if let Some(captures) = PUT_THIS_PANCAKE_ON_TOP_REGEX.captures_iter(line).next() {
                    return Ok(BorrowedCommand::PutThisPancakeOnTop(
                        captures.get(1).unwrap().as_str(),
                    ));
                }
                if let Some(captures) = LABEL_REGEX.captures_iter(line).next() {
                    return Ok(BorrowedCommand::Label(captures.get(1).unwrap().as_str()));
                }
                if let Some(captures) = IF_THE_PANCAKE_ISNT_TASTY_GO_OVER_TO_REGEX
                    .captures_iter(line)
                    .next()
                {
                    return Ok(BorrowedCommand::IfThePancakeIsntTastyGoOverTo(
                        captures.get(1).unwrap().as_str(),
                    ));
                }
                if let Some(captures) = IF_THE_PANCAKE_IS_TASTY_GO_OVER_TO_REGEX
                    .captures_iter(line)
                    .next()
                {
                    return Ok(BorrowedCommand::IfThePancakeIsTastyGoOverTo(
                        captures.get(1).unwrap().as_str(),
                    ));
                }

                Err(ParseCommandError::new(line))
            }
        }
    }
    /// Converts this command into an [`OwnedCommand`](./enum.OwnedCommand.html) heap allocating the referenced [`str`](https://doc.rust-lang.org/std/str/)s.
    pub fn to_owned(&self) -> OwnedCommand {
        match self {
            BorrowedCommand::PutThisPancakeOnTop(adj) => {
                OwnedCommand::PutThisPancakeOnTop(adj.to_string())
            }
            BorrowedCommand::EatThePancakeOnTop => OwnedCommand::EatThePancakeOnTop,
            BorrowedCommand::PutTheTopPancakesTogether => OwnedCommand::PutTheTopPancakesTogether,
            BorrowedCommand::GiveMeAPancake => OwnedCommand::GiveMeAPancake,
            BorrowedCommand::HowAboutAHotcake => OwnedCommand::HowAboutAHotcake,
            BorrowedCommand::ShowMeAPancake => OwnedCommand::ShowMeAPancake,
            BorrowedCommand::TakeFromTheTopPancakes => OwnedCommand::TakeFromTheTopPancakes,
            BorrowedCommand::FlipThePancakesOnTop => OwnedCommand::FlipThePancakesOnTop,
            BorrowedCommand::PutAnotherPancakeOnTop => OwnedCommand::PutAnotherPancakeOnTop,
            BorrowedCommand::Label(label) => OwnedCommand::Label(label.to_string()),
            BorrowedCommand::IfThePancakeIsntTastyGoOverTo(label) => {
                OwnedCommand::IfThePancakeIsntTastyGoOverTo(label.to_string())
            }
            BorrowedCommand::IfThePancakeIsTastyGoOverTo(label) => {
                OwnedCommand::IfThePancakeIsTastyGoOverTo(label.to_string())
            }
            BorrowedCommand::PutSyrupOnThePancakes => OwnedCommand::PutSyrupOnThePancakes,
            BorrowedCommand::PutButterOnThePancakes => OwnedCommand::PutButterOnThePancakes,
            BorrowedCommand::TakeOffTheSyrup => OwnedCommand::TakeOffTheSyrup,
            BorrowedCommand::TakeOffTheButter => OwnedCommand::TakeOffTheButter,
            BorrowedCommand::EatAllOfThePancakes => OwnedCommand::EatAllOfThePancakes,
        }
    }
}

/// An enum representing a pancakestack command.
/// Labels and pancake adjectives are stored in [`String`](https://doc.rust-lang.org/std/string/struct.String.html)s .
/// See [`BorrowedCommand`](./enum.BorrowedCommand.html) for a version that uses [`str`](https://doc.rust-lang.org/std/str/)s.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum OwnedCommand {
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
    EatAllOfThePancakes,
}

impl OwnedCommand {
    /// Parses the given line as a pancake stack command.
    /// The command will clone parts of the string (labels and adjectives).
    pub fn from_line(line: &'_ str) -> Result<Self, ParseCommandError<'_>> {
        BorrowedCommand::from_line(line).map(|e| e.to_owned())
    }

    /// Converts this command into an [`BorrowedCommand`](./enum.BorrowedCommand.html) referencing the strings in the original command.
    pub fn borrow(&'_ self) -> BorrowedCommand<'_> {
        match self {
            OwnedCommand::PutThisPancakeOnTop(adj) => BorrowedCommand::PutThisPancakeOnTop(&adj),
            OwnedCommand::EatThePancakeOnTop => BorrowedCommand::EatThePancakeOnTop,
            OwnedCommand::PutTheTopPancakesTogether => BorrowedCommand::PutTheTopPancakesTogether,
            OwnedCommand::GiveMeAPancake => BorrowedCommand::GiveMeAPancake,
            OwnedCommand::HowAboutAHotcake => BorrowedCommand::HowAboutAHotcake,
            OwnedCommand::ShowMeAPancake => BorrowedCommand::ShowMeAPancake,
            OwnedCommand::TakeFromTheTopPancakes => BorrowedCommand::TakeFromTheTopPancakes,
            OwnedCommand::FlipThePancakesOnTop => BorrowedCommand::FlipThePancakesOnTop,
            OwnedCommand::PutAnotherPancakeOnTop => BorrowedCommand::PutAnotherPancakeOnTop,
            OwnedCommand::Label(label) => BorrowedCommand::Label(&label),
            OwnedCommand::IfThePancakeIsntTastyGoOverTo(label) => {
                BorrowedCommand::IfThePancakeIsntTastyGoOverTo(&label)
            }
            OwnedCommand::IfThePancakeIsTastyGoOverTo(label) => {
                BorrowedCommand::IfThePancakeIsTastyGoOverTo(&label)
            }
            OwnedCommand::PutSyrupOnThePancakes => BorrowedCommand::PutSyrupOnThePancakes,
            OwnedCommand::PutButterOnThePancakes => BorrowedCommand::PutButterOnThePancakes,
            OwnedCommand::TakeOffTheSyrup => BorrowedCommand::TakeOffTheSyrup,
            OwnedCommand::TakeOffTheButter => BorrowedCommand::TakeOffTheButter,
            OwnedCommand::EatAllOfThePancakes => BorrowedCommand::EatAllOfThePancakes,
        }
    }
}

#[derive(Debug)]
pub struct ParseCommandError<'line> {
    line: &'line str,
}
impl<'line> ParseCommandError<'line> {
    pub fn new(line: &'line str) -> Self {
        ParseCommandError { line }
    }
    pub fn line(&self) -> &str {
        self.line
    }
}
impl Display for ParseCommandError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to parse command: {}", self.line())
    }
}
impl std::error::Error for ParseCommandError<'_> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
