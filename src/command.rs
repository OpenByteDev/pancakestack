use regex::Regex;
use std::fmt::{self, Display};

lazy_static! {
    static ref PUT_THIS_PANCAKE_ON_TOP_REGEX: Regex =
        Regex::new(r"^Put this (\S*) pancake on top!$").unwrap();
    static ref EAT_THE_PANCAKE_ON_TOP_REGEX: Regex =
        Regex::new(r"^Eat the pancake on top!$").unwrap();
    static ref PUT_THE_TOP_PANCAKES_TOGETHER_REGEX: Regex =
        Regex::new(r"^Put the top pancakes together!$").unwrap();
    static ref GIVE_ME_A_PANCAKE_REGEX: Regex = Regex::new(r"^Give me a pancake!$").unwrap();
    static ref HOW_ABOUT_A_HOTCAKE_REGEX: Regex = Regex::new(r"^How about a hotcake\?$").unwrap();
    static ref SHOW_ME_A_PANCAKE_REGEX: Regex = Regex::new(r"^Show me a pancake!$").unwrap();
    static ref TAKE_FROM_THE_TOP_PANCAKES_REGEX: Regex =
        Regex::new(r"^Take from the top pancakes!$").unwrap();
    static ref FLIP_THE_PANCAKES_ON_TOP_REGEX: Regex =
        Regex::new(r"^Flip the pancakes on top!$").unwrap();
    static ref PUT_ANOTHER_PANCAKE_ON_TOP_REGEX: Regex =
        Regex::new(r"^Put another pancake on top!$").unwrap();
    static ref LABEL_REGEX: Regex = Regex::new(r"^\[(.+)\]$").unwrap();
    static ref IF_THE_PANCAKE_ISNT_TASTY_GO_OVER_TO_REGEX: Regex =
        Regex::new("^If the pancake isn't tasty, go over to \"(.*)\"\\.$").unwrap();
    static ref IF_THE_PANCAKE_IS_TASTY_GO_OVER_TO_REGEX: Regex =
        Regex::new("^If the pancake is tasty, go over to \"(.*)\"\\.$").unwrap();
    static ref PUT_SYRUP_ON_THE_PANCAKES_REGEX: Regex =
        Regex::new(r"^Put syrup on the pancakes!$").unwrap();
    static ref PUT_BUTTER_ON_THE_PANCAKES_REGEX: Regex =
        Regex::new(r"^Put butter on the pancakes!$").unwrap();
    static ref TAKE_OFF_THE_SYRUP_REGEX: Regex = Regex::new(r"^Take off the syrup!$").unwrap();
    static ref TAKE_OFF_THE_BUTTER_REGEX: Regex = Regex::new(r"^Take off the butter!$").unwrap();
    static ref EAT_ALL_OF_THE_PANCAKES_REGEX: Regex =
        Regex::new(r"^Eat all of the pancakes!$").unwrap();
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
        if let Some(captures) = PUT_THIS_PANCAKE_ON_TOP_REGEX.captures_iter(line).next() {
            return Ok(BorrowedCommand::PutThisPancakeOnTop(
                captures.get(1).unwrap().as_str(),
            ));
        }
        if EAT_THE_PANCAKE_ON_TOP_REGEX.is_match(line) {
            return Ok(BorrowedCommand::EatThePancakeOnTop);
        }
        if PUT_THE_TOP_PANCAKES_TOGETHER_REGEX.is_match(line) {
            return Ok(BorrowedCommand::PutTheTopPancakesTogether);
        }
        if GIVE_ME_A_PANCAKE_REGEX.is_match(line) {
            return Ok(BorrowedCommand::GiveMeAPancake);
        }
        if HOW_ABOUT_A_HOTCAKE_REGEX.is_match(line) {
            return Ok(BorrowedCommand::HowAboutAHotcake);
        }
        if SHOW_ME_A_PANCAKE_REGEX.is_match(line) {
            return Ok(BorrowedCommand::ShowMeAPancake);
        }
        if TAKE_FROM_THE_TOP_PANCAKES_REGEX.is_match(line) {
            return Ok(BorrowedCommand::TakeFromTheTopPancakes);
        }
        if FLIP_THE_PANCAKES_ON_TOP_REGEX.is_match(line) {
            return Ok(BorrowedCommand::FlipThePancakesOnTop);
        }
        if PUT_ANOTHER_PANCAKE_ON_TOP_REGEX.is_match(line) {
            return Ok(BorrowedCommand::PutAnotherPancakeOnTop);
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
        if PUT_SYRUP_ON_THE_PANCAKES_REGEX.is_match(line) {
            return Ok(BorrowedCommand::PutSyrupOnThePancakes);
        }
        if PUT_BUTTER_ON_THE_PANCAKES_REGEX.is_match(line) {
            return Ok(BorrowedCommand::PutButterOnThePancakes);
        }
        if TAKE_OFF_THE_SYRUP_REGEX.is_match(line) {
            return Ok(BorrowedCommand::TakeOffTheSyrup);
        }
        if TAKE_OFF_THE_BUTTER_REGEX.is_match(line) {
            return Ok(BorrowedCommand::TakeOffTheButter);
        }
        if EAT_ALL_OF_THE_PANCAKES_REGEX.is_match(line) {
            return Ok(BorrowedCommand::EatAllOfThePancakes);
        }
        Err(ParseCommandError::new(line))
    }
    /// Converts this command into an [`OwnedCommand`](./enum.OwnedCommand.html) heap allocating the referenced [`str`](https://doc.rust-lang.org/std/str/)s.
    pub fn to_owned(self) -> OwnedCommand {
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
    pub fn from_line<'a>(line: &'a str) -> Result<Self, ParseCommandError<'a>> {
        BorrowedCommand::from_line(line).map(|e| e.to_owned())
    }

    /// Converts this command into an [`BorrowedCommand`](./enum.BorrowedCommand.html) referencing the strings in the original command.
    pub fn borrow<'a>(&'a self) -> BorrowedCommand<'a> {
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