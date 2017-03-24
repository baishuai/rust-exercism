use std::collections::HashSet;
use std::ascii::AsciiExt;
pub fn is_pangram(sentence: &str) -> bool {
    sentence.to_uppercase()
        .chars()
        .filter(|x| x.is_alphabetic() && x.is_ascii())
        .collect::<HashSet<_>>()
        .len() == 26
}