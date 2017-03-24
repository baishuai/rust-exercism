use std::ascii::AsciiExt;
// Plain:  abcdefghijklmnopqrstuvwxyz
// Cipher: zyxwvutsrqponmlkjihgfedcba

const KEY: u8 = 'a' as u8 + 'z' as u8;

pub fn encode(plain: &str) -> String {
    plain.to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_ascii() && c.is_alphanumeric())
        .map(|c| if c.is_alphabetic() {
            (KEY - c as u8) as char
        } else {
            c
        })
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|part| part.iter().cloned().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn decode(cipher: &str) -> String {
    encode(cipher)
        .split_whitespace()
        .collect::<String>()
}
