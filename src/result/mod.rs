use crate::error::BError;

/// The 🅱️ result type.
/// ```rust
/// use b_ext::{B, result::Result};
///
/// fn main() -> Result<()> {
///     let b_emoji = B::try_from("🅱️")?;
///     println!("{}", b_emoji);
///     Ok(())
/// }
pub type Result<T> = std::result::Result<T, BError>;
