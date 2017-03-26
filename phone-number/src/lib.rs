pub fn number(phone: &str) -> Option<String> {
    let num = phone.chars().filter(|c| c.is_numeric()).collect::<String>();
    match (num.len(), num.chars().nth(0)) {
        (10, _) => Some(num),
        (11, Some('1')) => Some(num.chars().skip(1).collect::<String>()),
        (_, _) => None,
    }
}

pub fn area_code(phone: &str) -> Option<String> {
    number(phone).map(|s| s[..3].to_owned())
}

pub fn pretty_print(phone: &str) -> String {
    number(phone)
        .map(|s| format!("({}) {}-{}", &s[..3], &s[3..6], &s[6..]))
        .unwrap_or("invalid".to_owned())
}
