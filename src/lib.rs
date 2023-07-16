//! # The Extended 🅱️ Library
//! Give the gift of 🅱️, now with more features!

/// Error handling for the `b-ext` crate.
pub mod error;
/// The 🅱️ result type.
pub mod result;
#[cfg(test)]
pub mod tests;

use error::BError;
use std::{fmt::Display, str::FromStr};

pub use b::B;

/// The 🅱️ enum. Includes emoji, ASCII, and many Unicode variants to suit your needs.
/// ```rust
/// // Use the `Emoji` variant in your cool Rust project.
/// use b_ext::B;
///
/// let b_emoji = B::Emoji;
/// println!("{}", b_emoji);
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum B {
    /// The 🅱️ emoji. [U+1F171 U+FE0F]
    #[default]
    Emoji,
    /// The lowercase ASCII `b`. [U+0062]
    ASCIILowercase,
    /// The uppercase ASCII `B`. [U+0042]
    ASCIIUppercase,
    /// The 🄱 squared Unicode character. [U+1F131]
    Squared,
    /// The 🅱 negative squared Unicode character. [U+1F171]
    NegativeSquared,
}

impl Display for B {
    /// Formats a 🅱️.
    /// ```rust
    /// use b_ext::B;
    ///
    /// let b_emoji = B::Emoji;
    /// assert_eq!(format!("{}", b_emoji), "🅱️");
    /// ```
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
    /// Converts a string slice to a [`enum@B`]. Returns a [`BError`] if the string is not a valid 🅱️ string representation.
    /// ```rust
    /// use b_ext::B;
    ///
    /// assert!(B::try_from("🅱️").is_ok());
    /// assert!(B::try_from("not b").is_err());
    /// ```
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "🅱️" => Ok(B::Emoji),
            "b" => Ok(B::ASCIILowercase),
            "B" => Ok(B::ASCIIUppercase),
            "🄱" => Ok(B::Squared),
            "🅱" => Ok(B::NegativeSquared),
            _ => Err(BError::InvalidB(value.to_string())),
        }
    }
}

impl TryFrom<char> for B {
    type Error = BError;
    /// Converts a [`char`] to a [`enum@B`]. Returns a [`BError`] if the char is not a valid 🅱️ string representation.
    /// ```rust
    /// use b_ext::B;
    ///
    /// assert!(B::try_from('🅱').is_ok());
    /// assert!(B::try_from('c').is_err());
    /// ```
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '🅱' => Ok(B::NegativeSquared),
            '🄱' => Ok(B::Squared),
            'B' => Ok(B::ASCIIUppercase),
            'b' => Ok(B::ASCIILowercase),
            _ => Err(BError::InvalidB(value.to_string())),
        }
    }
}

impl TryFrom<&String> for B {
    type Error = BError;
    /// Converts a [`String`] reference to a [`enum@B`]. Returns a [`BError`] if the string is not a valid 🅱️ string representation.
    /// ```rust
    /// use b_ext::B;
    ///
    /// assert!(B::try_from(&String::from("🅱️")).is_ok());
    /// assert!(B::try_from(&String::from("not b")).is_err());
    /// ```
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        B::try_from(value.as_str())
    }
}

impl FromStr for B {
    type Err = BError;
    /// Parses a 🅱️ from a string slice.
    /// ```rust
    /// use b_ext::B;
    ///
    /// let b_emoji = B::try_from("🅱️").unwrap();
    /// assert_eq!(b_emoji, B::Emoji);
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        B::try_from(s)
    }
}

/// 🅱️ macro to put the 🅱️ emoji in your code at compile time.
/// ```rust
/// // Use the `b!()` macro to include the 🅱️ emoji.
/// use b_ext::b;
///
/// let b_emoji = b!();
/// println!("{}", b_emoji);
/// ```
#[macro_export]
macro_rules! b {
    () => {
        b_ext::B::Emoji
    };
}
