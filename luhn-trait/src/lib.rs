pub struct Luhn(String);

pub trait ValidLuhn {
    fn valid_luhn(&self) -> bool;
}

impl<T: ToString> ValidLuhn for T {
    fn valid_luhn(&self) -> bool {
        Luhn(self.to_string()).is_valid()
    }
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let ref num = self.0;
        if num.chars()
            .filter(|c| !c.is_whitespace())
            .any(|c| !c.is_numeric()) ||
           num.chars()
            .filter(|c| !c.is_whitespace())
            .take(2)
            .count() < 2 {
            return false;
        }
        num.chars()
            .filter(|c| !c.is_whitespace())
            .rev()
            .enumerate()
            .map(|(i, c)| {
                let v = ((c as u32) - '0' as u32) * 2;
                if i % 2 == 1 {
                    if v > 9 { v - 9 } else { v }
                } else {
                    v / 2
                }
            })
            .sum::<u32>() % 10 == 0
    }
}
