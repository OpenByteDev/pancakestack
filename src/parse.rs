use lazy_regex::{lazy_regex, Lazy};
use regex::Regex;
use std::{
    borrow::Cow,
    fmt::{self, Display},
};

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
#[must_use]
pub fn parse_program_str(program: &str) -> Vec<Command> {
    program
        .lines()
        .filter_map(|line| Command::from_line(line).ok())
        .collect()
}

/// An enum representing a pancakestack command.
/// Labels and pancake adjectives are stored in [`str`](https://doc.rust-lang.org/std/str/)s .
/// See [`Command`](./enum.Command.html) for a version that uses [`String`](https://doc.rust-lang.org/std/string/struct.String.html)s.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Command<'a> {
    PutThisPancakeOnTop(Cow<'a, str>),
    EatThePancakeOnTop,
    PutTheTopPancakesTogether,
    GiveMeAPancake,
    HowAboutAHotcake,
    ShowMeAPancake,
    TakeFromTheTopPancakes,
    FlipThePancakesOnTop,
    PutAnotherPancakeOnTop,
    Label(Cow<'a, str>),
    IfThePancakeIsntTastyGoOverTo(Cow<'a, str>),
    IfThePancakeIsTastyGoOverTo(Cow<'a, str>),
    PutSyrupOnThePancakes,
    PutButterOnThePancakes,
    TakeOffTheSyrup,
    TakeOffTheButter,
    EatAllOfThePancakes,
}

static PUT_THIS_PANCAKE_ON_TOP_REGEX: Lazy<Regex> =
    lazy_regex!(r"^Put this (\S*) pancake on top!$");
static LABEL_REGEX: Lazy<Regex> = lazy_regex!(r"^\[(.+)\]$");
static IF_THE_PANCAKE_ISNT_TASTY_GO_OVER_TO_REGEX: Lazy<Regex> =
    lazy_regex!("^If the pancake isn't tasty, go over to \"(.*)\"\\.$");
static IF_THE_PANCAKE_IS_TASTY_GO_OVER_TO_REGEX: Lazy<Regex> =
    lazy_regex!("^If the pancake is tasty, go over to \"(.*)\"\\.$");

impl<'a> Command<'a> {
    /// Parses the given line as a pancake stack command.
    /// The command will reference the strings contents.
    ///
    /// # Errors
    /// Will return [`Err`] if the given line cannot be parsed as a command.
    pub fn from_line(line: &'a str) -> Result<Self, CommandParseError<'a>> {
        match line {
            "Eat the pancake on top!" => Ok(Self::EatThePancakeOnTop),
            "Put the top pancakes together!" => Ok(Self::PutTheTopPancakesTogether),
            "Give me a pancake!" => Ok(Self::GiveMeAPancake),
            "How about a hotcake?" => Ok(Self::HowAboutAHotcake),
            "Show me a pancake!" => Ok(Self::ShowMeAPancake),
            "Take from the top pancakes!" => Ok(Self::TakeFromTheTopPancakes),
            "Flip the pancakes on top!" => Ok(Self::FlipThePancakesOnTop),
            "Put another pancake on top!" => Ok(Self::PutAnotherPancakeOnTop),
            "Put syrup on the pancakes!" => Ok(Self::PutSyrupOnThePancakes),
            "Put butter on the pancakes!" => Ok(Self::PutButterOnThePancakes),
            "Take off the syrup!" => Ok(Self::TakeOffTheSyrup),
            "Take off the butter!" => Ok(Self::TakeOffTheButter),
            "Eat all of the pancakes!" => Ok(Self::EatAllOfThePancakes),
            _ => {
                if let Some(captures) = PUT_THIS_PANCAKE_ON_TOP_REGEX.captures_iter(line).next() {
                    return Ok(Self::PutThisPancakeOnTop(
                        captures.get(1).unwrap().as_str().into(),
                    ));
                }
                if let Some(captures) = LABEL_REGEX.captures_iter(line).next() {
                    return Ok(Self::Label(captures.get(1).unwrap().as_str().into()));
                }
                if let Some(captures) = IF_THE_PANCAKE_ISNT_TASTY_GO_OVER_TO_REGEX
                    .captures_iter(line)
                    .next()
                {
                    return Ok(Self::IfThePancakeIsntTastyGoOverTo(
                        captures.get(1).unwrap().as_str().into(),
                    ));
                }
                if let Some(captures) = IF_THE_PANCAKE_IS_TASTY_GO_OVER_TO_REGEX
                    .captures_iter(line)
                    .next()
                {
                    return Ok(Self::IfThePancakeIsTastyGoOverTo(
                        captures.get(1).unwrap().as_str().into(),
                    ));
                }

                Err(CommandParseError::new(line))
            }
        }
    }

    /// Creates a new owned version of this command, heap allocating the referenced [`str`]s.
    #[must_use]
    pub fn to_owned(&self) -> Command<'static> {
        match self {
            Self::PutThisPancakeOnTop(adj) => Command::PutThisPancakeOnTop(adj.to_string().into()),
            Self::EatThePancakeOnTop => Command::EatThePancakeOnTop,
            Self::PutTheTopPancakesTogether => Command::PutTheTopPancakesTogether,
            Self::GiveMeAPancake => Command::GiveMeAPancake,
            Self::HowAboutAHotcake => Command::HowAboutAHotcake,
            Self::ShowMeAPancake => Command::ShowMeAPancake,
            Self::TakeFromTheTopPancakes => Command::TakeFromTheTopPancakes,
            Self::FlipThePancakesOnTop => Command::FlipThePancakesOnTop,
            Self::PutAnotherPancakeOnTop => Command::PutAnotherPancakeOnTop,
            Self::Label(label) => Command::Label(label.to_string().into()),
            Self::IfThePancakeIsntTastyGoOverTo(label) => {
                Command::IfThePancakeIsntTastyGoOverTo(label.to_string().into())
            }
            Self::IfThePancakeIsTastyGoOverTo(label) => {
                Command::IfThePancakeIsTastyGoOverTo(label.to_string().into())
            }
            Self::PutSyrupOnThePancakes => Command::PutSyrupOnThePancakes,
            Self::PutButterOnThePancakes => Command::PutButterOnThePancakes,
            Self::TakeOffTheSyrup => Command::TakeOffTheSyrup,
            Self::TakeOffTheButter => Command::TakeOffTheButter,
            Self::EatAllOfThePancakes => Command::EatAllOfThePancakes,
        }
    }

    /// Consumes this command, converting it to an owned version, heap allocating the referenced [`str`]s if they are not already owned.
    #[must_use]
    pub fn into_owned(self) -> Command<'static> {
        match self {
            Self::PutThisPancakeOnTop(adj) => Command::PutThisPancakeOnTop(adj.into_owned().into()),
            Self::EatThePancakeOnTop => Command::EatThePancakeOnTop,
            Self::PutTheTopPancakesTogether => Command::PutTheTopPancakesTogether,
            Self::GiveMeAPancake => Command::GiveMeAPancake,
            Self::HowAboutAHotcake => Command::HowAboutAHotcake,
            Self::ShowMeAPancake => Command::ShowMeAPancake,
            Self::TakeFromTheTopPancakes => Command::TakeFromTheTopPancakes,
            Self::FlipThePancakesOnTop => Command::FlipThePancakesOnTop,
            Self::PutAnotherPancakeOnTop => Command::PutAnotherPancakeOnTop,
            Self::Label(label) => Command::Label(label.into_owned().into()),
            Self::IfThePancakeIsntTastyGoOverTo(label) => {
                Command::IfThePancakeIsntTastyGoOverTo(label.into_owned().into())
            }
            Self::IfThePancakeIsTastyGoOverTo(label) => {
                Command::IfThePancakeIsTastyGoOverTo(label.into_owned().into())
            }
            Self::PutSyrupOnThePancakes => Command::PutSyrupOnThePancakes,
            Self::PutButterOnThePancakes => Command::PutButterOnThePancakes,
            Self::TakeOffTheSyrup => Command::TakeOffTheSyrup,
            Self::TakeOffTheButter => Command::TakeOffTheButter,
            Self::EatAllOfThePancakes => Command::EatAllOfThePancakes,
        }
    }

    /// Creates a new borrowed version of this command.
    #[must_use]
    pub fn borrow(&self) -> Command<'_> {
        match self {
            Self::PutThisPancakeOnTop(adj) => Command::PutThisPancakeOnTop(adj.as_ref().into()),
            Self::EatThePancakeOnTop => Command::EatThePancakeOnTop,
            Self::PutTheTopPancakesTogether => Command::PutTheTopPancakesTogether,
            Self::GiveMeAPancake => Command::GiveMeAPancake,
            Self::HowAboutAHotcake => Command::HowAboutAHotcake,
            Self::ShowMeAPancake => Command::ShowMeAPancake,
            Self::TakeFromTheTopPancakes => Command::TakeFromTheTopPancakes,
            Self::FlipThePancakesOnTop => Command::FlipThePancakesOnTop,
            Self::PutAnotherPancakeOnTop => Command::PutAnotherPancakeOnTop,
            Self::Label(label) => Command::Label(label.as_ref().into()),
            Self::IfThePancakeIsntTastyGoOverTo(label) => {
                Command::IfThePancakeIsntTastyGoOverTo(label.as_ref().into())
            }
            Self::IfThePancakeIsTastyGoOverTo(label) => {
                Command::IfThePancakeIsTastyGoOverTo(label.as_ref().into())
            }
            Self::PutSyrupOnThePancakes => Command::PutSyrupOnThePancakes,
            Self::PutButterOnThePancakes => Command::PutButterOnThePancakes,
            Self::TakeOffTheSyrup => Command::TakeOffTheSyrup,
            Self::TakeOffTheButter => Command::TakeOffTheButter,
            Self::EatAllOfThePancakes => Command::EatAllOfThePancakes,
        }
    }
}

impl Display for Command<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PutThisPancakeOnTop(adj) => write!(f, "Put this {adj} pancake on top!"),
            Self::EatThePancakeOnTop => write!(f, "Eat the pancake on top!"),
            Self::PutTheTopPancakesTogether => write!(f, "Put the top pancakes together!"),
            Self::GiveMeAPancake => write!(f, "Give me a pancake!"),
            Self::HowAboutAHotcake => write!(f, "How about a hotcake?"),
            Self::ShowMeAPancake => write!(f, "Show me a pancake!"),
            Self::TakeFromTheTopPancakes => write!(f, "Take from the top pancakes!"),
            Self::FlipThePancakesOnTop => write!(f, "Flip the pancakes on top!"),
            Self::PutAnotherPancakeOnTop => write!(f, "Put another pancake on top!"),
            Self::Label(label) => write!(f, "[{label}]"),
            Self::IfThePancakeIsntTastyGoOverTo(label) => {
                write!(f, "If the pancake isn't tasty, go over to \"{label}\"\\.")
            }
            Self::IfThePancakeIsTastyGoOverTo(label) => {
                write!(f, "If the pancake is tasty, go over to \"{label}\"\\.")
            }
            Self::PutSyrupOnThePancakes => write!(f, "Put syrup on the pancakes!"),
            Self::PutButterOnThePancakes => write!(f, "Put butter on the pancakes!"),
            Self::TakeOffTheSyrup => write!(f, "Take off the syrup!"),
            Self::TakeOffTheButter => write!(f, "Take off the butter!"),
            Self::EatAllOfThePancakes => write!(f, "Eat all of the pancakes!"),
        }
    }
}

#[derive(Debug)]
pub struct CommandParseError<'line> {
    line: &'line str,
}
impl<'line> CommandParseError<'line> {
    #[must_use]
    pub fn new(line: &'line str) -> Self {
        CommandParseError { line }
    }

    #[must_use]
    pub fn line(&self) -> &str {
        self.line
    }
}
impl Display for CommandParseError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to parse command: {}", self.line())
    }
}
impl std::error::Error for CommandParseError<'_> {}
