pub mod error;

use error::BError;

use std::{str::FromStr, fmt::Display};
use b::B;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum B {
    #[default]
    Emoji,
    ASCIILowercase,
    ASCIIUppercase,
    Squared,
    NegativeSquared,
}

impl Display for B {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            B::Emoji => write!(f, "{}", B),
            B::ASCIILowercase => write!(f, "b"),
            B::ASCIIUppercase => write!(f, "B"),
            B::Squared => write!(f, "🄱"),
            B::NegativeSquared => write!(f, "🅱"),
        }
    }
}

impl TryFrom<&str> for B {
    type Error = BError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "🅱️" => Ok(B::Emoji),
            "b" => Ok(B::ASCIILowercase),
            "B" => Ok(B::ASCIIUppercase),
            "🄱" => Ok(B::Squared),
            "🅱" => Ok(B::NegativeSquared),
            _ => Err(BError::InvalidB),
        }
    }
}

impl TryFrom<char> for B {
    type Error = BError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '🅱' => Ok(B::NegativeSquared),
            '🄱' => Ok(B::Squared),
            'B' => Ok(B::ASCIIUppercase),
            'b' => Ok(B::ASCIILowercase),
            _ => Err(BError::InvalidB),
        }
    }
}

impl FromStr for B {
    type Err = BError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        B::try_from(s)
    }
}

#[macro_export]
macro_rules! b {
    () => {
        B::Emoji
    };
}
