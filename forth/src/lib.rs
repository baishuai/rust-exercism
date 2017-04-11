use std::collections::HashMap;
use std::ops::{Add, Div, Mul, Sub};

pub type Value = i32;
pub type ForthResult = Result<(), Error>;

pub struct Forth {
    stack: Vec<Value>,
    defines: HashMap<String, Vec<String>>,
}


#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            stack: Vec::new(),
            defines: HashMap::new(),
        }
    }

    pub fn format_stack(&self) -> String {
        self.stack
            .iter()
            .map(|&i| i.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        let mut command_stack: Vec<String> = input.to_lowercase()
            .split(|c: char| c.is_whitespace() || c.is_control())
            .rev()
            .map(|s| String::from(s))
            .collect();

        println!("{:?}", command_stack);
        self.eval_tokens(&mut command_stack)
    }

    fn eval_tokens(&mut self, command_stack: &mut Vec<String>) -> ForthResult {


        while let Some(w) = command_stack.pop() {
            match w.as_str() {
                "+" => self.handle_bin_op(Value::add)?,
                "-" => self.handle_bin_op(Value::sub)?,
                "*" => self.handle_bin_op(Value::mul)?,
                "/" => {
                    if self.stack.len() >= 2 && self.stack.last().unwrap().clone() == 0 {
                        return Err(Error::DivisionByZero);
                    }
                    self.handle_bin_op(Value::div)?
                }
                _ if self.defines.contains_key(&w) => {
                    let def = self.defines.get(&w).unwrap();
                    for d in def {
                        command_stack.push(d.clone());
                    }
                    continue;
                }
                "dup" => {
                    if let Some(a) = self.stack.pop() {
                        self.stack.push(a);
                        self.stack.push(a);
                    } else {
                        return Err(Error::StackUnderflow);
                    }
                }
                "drop" => {
                    if let Some(_) = self.stack.pop() {
                        ()
                    } else {
                        return Err(Error::StackUnderflow);
                    }
                }
                "swap" => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        self.stack.push(b);
                        self.stack.push(a);
                    } else {
                        return Err(Error::StackUnderflow);
                    }
                }
                "over" => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        self.stack.push(a);
                        self.stack.push(b);
                        self.stack.push(a);
                    } else {
                        return Err(Error::StackUnderflow);
                    }
                }
                _ if w.parse::<Value>().is_ok() => self.stack.push(w.parse::<Value>().unwrap()),
                ":" => {
                    let t = command_stack.pop();
                    if t == None {
                        return Err(Error::InvalidWord);
                    }
                    let word: String = t.unwrap();
                    if word.parse::<Value>().is_ok() {
                        return Err(Error::InvalidWord);
                    }

                    let mut definition: Vec<String> = Vec::new();
                    loop {
                        let t = command_stack.pop();
                        if t == None {
                            return Err(Error::InvalidWord);
                        }
                        let word = t.unwrap();
                        if word == ";" {
                            break;
                        }
                        definition.push(word);
                    }
                    if definition.len() == 0 {
                        return Err(Error::InvalidWord);
                    }
                    definition.reverse();
                    self.defines.insert(word, definition);
                }
                _ => {
                    return Err(Error::UnknownWord);
                }
            }
        }
        Ok(())
    }

    fn handle_bin_op(&mut self, op: fn(Value, Value) -> Value) -> ForthResult {
        if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
            self.stack.push(op(a, b));
            Ok(())
        } else {
            Err(Error::StackUnderflow)
        }
    }
}
