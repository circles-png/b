#[test]
fn converting_from_str_works() {
    use crate::B;

    let b_emoji = B::try_from("🅱️").unwrap();
    assert_eq!(b_emoji, B::Emoji);
    let b_ascii_lowercase = B::try_from("b").unwrap();
    assert_eq!(b_ascii_lowercase, B::ASCIILowercase);
    let b_ascii_uppercase = B::try_from("B").unwrap();
    assert_eq!(b_ascii_uppercase, B::ASCIIUppercase);
    let b_squared = B::try_from("🄱").unwrap();
    assert_eq!(b_squared, B::Squared);
    let b_negative_squared = B::try_from("🅱").unwrap();
    assert_eq!(b_negative_squared, B::NegativeSquared);
}

#[test]
fn display_and_to_string_works() {
    use crate::B;

    let b_emoji = B::Emoji;
    assert_eq!(format!("{}", b_emoji), "🅱️");
    let b_ascii_lowercase = B::ASCIILowercase;
    assert_eq!(format!("{}", b_ascii_lowercase), "b");
    let b_ascii_uppercase = B::ASCIIUppercase;
    assert_eq!(format!("{}", b_ascii_uppercase), "B");
    let b_squared = B::Squared;
    assert_eq!(format!("{}", b_squared), "🄱");
    let b_negative_squared = B::NegativeSquared;
    assert_eq!(format!("{}", b_negative_squared), "🅱");
}
