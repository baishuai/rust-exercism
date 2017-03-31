use std::ops::{Add, Div, Mul, Sub};


#[derive(Debug)]
pub struct WordProblem {
    command: String,
    ans: Option<i32>,
}


fn parser(command: &str) -> Option<i32> {
    let tokens = command.trim_right_matches('?')
        .split_whitespace().skip(2);

    let mut res = 0;
    let mut op: fn(i32, i32) -> i32 = i32::add;

    for token in tokens {
        match token.parse::<i32>() {
            Ok(num) => {
                res = op(res, num);
            }
            Err(_) => {
                if token == "by" {
                    continue;
                }
                op = match token {
                    "plus" => i32::add,
                    "minus" => i32::sub,
                    "multiplied" => i32::mul,
                    "divided" => i32::div,
                    _ => return None
                }
            }
        }
    }
    Some(res)
}


impl WordProblem {
    pub fn new(c: &str) -> WordProblem {
        WordProblem {
            command: String::from(c),
            ans: parser(c)
        }
    }

    pub fn answer(&self) -> Result<i32, ()> {
        self.ans.ok_or(())
    }
}
