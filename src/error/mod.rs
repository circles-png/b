use std::{error::Error, fmt::Display};

/// The 🅱️ error type. Handles invalid 🅱️s in conversions.
///
/// ```rust
/// use b_ext::{error::BError, B};
///
/// let b_emoji = B::try_from("🅱️");
/// match b_emoji {
///     Ok(b) => println!("success! {}", b),
///     Err(e) => println!("error! {}", e),
/// }
/// ```
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum BError {
    /// The 🅱️ string representation is invalid.
    InvalidB(String),
}

impl Error for BError {}

impl Display for BError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BError::InvalidB(b) => write!(f, "invalid B string representation: {b}"),
        }
    }
}
