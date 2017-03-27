extern crate rand;

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Robot {
        let mut robot = Robot { name: "".to_string() };
        robot.reset_name();
        robot
    }
    pub fn name<'a>(&'a self) -> &'a str {
        self.name.as_str()
    }
    pub fn reset_name(&mut self) {
        let str = rand::random::<[u8; 5]>();

        let mut name = String::with_capacity(5);
        name.push((str[0] % 26 + b'A') as char);
        name.push((str[1] % 26 + b'A') as char);
        name.push((str[2] % 10 + b'0') as char);
        name.push((str[3] % 10 + b'0') as char);
        name.push((str[4] % 10 + b'0') as char);

        self.name = name;
    }
}