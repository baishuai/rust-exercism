pub fn reply(text: &str) -> &'static str {
    if text.is_empty() {
        "Fine. Be that way!"
    } else if text.ends_with('?') {
        "Sure."
    } else if text.contains(char::is_uppercase) && !text.contains(char::is_lowercase) {
        "Whoa, chill out!"
    } else {
        "Whatever."
    }
}
