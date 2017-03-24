
pub struct Roman {
    n: u32,
    r: String,
}

impl Roman {
    pub fn from(n: u32) -> Roman {
        Roman {
            n: n,
            r: Roman::to_roman(n),
        }
    }

    #[warn(dead_code)]
    fn to_arab(&self) -> u32 {
        self.n
    }

    fn to_roman(mut n: u32) -> String {
        println!("{:?}", n);

        let mut ans = String::new();

        let (mut div, mut pow) = (1000, 4);
        while div > 0 {
            let quo = n / div;
            n %= div;
            div /= 10;
            pow -= 1;
            if quo == 0 {
                continue;
            }
            let mut roman: String = match quo {
                    1 => "I", 
                    2 => "II", 
                    3 => "III",
                    4 => "IV",
                    5 => "V",
                    6 => "VI",
                    7 => "VII",
                    8 => "VIII",
                    9 => "IX",
                    _ => "",
                }
                .to_string();
            println!("{:?}, {}", roman, pow);
            for _ in 0..pow {
                roman = roman.chars()
                    .flat_map(|c| match c {
                        'I' => Some('X'),
                        'V' => Some('L'),
                        'X' => Some('C'),
                        'L' => Some('D'),
                        'C' => Some('M'),
                        _ => None,
                    })
                    .collect::<String>();
            }
            println!("{:?}", roman);
            ans += roman.as_str();
        }
        ans

    }
}

impl ToString for Roman {
    fn to_string(&self) -> String {
        self.r.clone()
    }
}