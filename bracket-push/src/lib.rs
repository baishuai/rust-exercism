
#[derive(Debug)]
pub struct Brackets {
    raw: String,
    valid: bool,
}

fn is_valid(text: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for c in text.chars()
        .filter(|&c| c == '(' || c == ')' || c == '[' || c == ']' || c == '{' || c == '}') {
        if c == '(' || c == '[' || c == '{' {
            stack.push(c);
        } else {
            match stack.pop() {
                Some('(') if c == ')' => {}
                Some('[') if c == ']' => {}
                Some('{') if c == '}' => {}
                _ => return false,
            };
        }
    }
    return stack.is_empty();
}
impl<'a> From<&'a str> for Brackets {
    fn from(text: &'a str) -> Brackets {
        Brackets {
            raw: String::from(text),
            valid: is_valid(text),
        }
    }
}

impl Brackets {
    pub fn are_balanced(&self) -> bool {
        self.valid
    }
}