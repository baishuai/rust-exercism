
pub fn rotate(text: &str, key: u8) -> String {
    text.chars()
        .map(|c| match c {
            'a'...'z' => ((((c as u8 - b'a') + key) % 26) + b'a') as char,
            'A'...'Z' => ((((c as u8 - b'A') + key) % 26) + b'A') as char,
            _ => c,
        })
        .collect::<String>()
}