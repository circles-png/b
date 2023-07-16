use b_ext::{B, result::Result};

fn main() -> Result<()>
{
    // Enum variants
    println!("`Emoji`:           {}", B::Emoji);
    println!("`ASCIILowercase`:  {}", B::ASCIILowercase);
    println!("`ASCIIUppercase`:  {}", B::ASCIIUppercase);
    println!("`Squared`:         {}", B::Squared);
    println!("`NegativeSquared`: {}", B::NegativeSquared);

    // Conversions
    println!("from a `&str`: {}", B::try_from("🅱️")?);
    println!("from a `char`: {}", B::try_from('🅱')?);
    println!("from a `String`: {}", B::try_from(&String::from("🅱️"))?);

    // Error handling
    println!("from a bad `&str`: {:?}", B::try_from("not b"));

    // Static re-export
    println!("`B`: {}", B);

    // Default
    println!("Default: {}", B::default());

    // Equality
    println!("`B::Emoji == B::Emoji`: {}", B::Emoji == B::Emoji);

    Ok(())
}
