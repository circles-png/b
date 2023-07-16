use std::{error::Error, fmt::Display};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum BError {
    InvalidB,
}

impl Error for BError {}

impl Display for BError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BError::InvalidB => write!(f, "Invalid B"),
        }
    }
}

pub type Result<T> = std::result::Result<T, BError>;
