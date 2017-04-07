use std::collections::HashMap;

type Mapping = HashMap<char, u8>;

#[derive(Debug)]
struct Puzzle {
    unique_chars: Vec<char>,
    left: Vec<String>,
    right: Vec<String>,
    leading_chars: Vec<char>,
}

fn part_value(s: &Vec<String>, map: &Mapping) -> u32 {
    s.into_iter()
        .map(|word| {
            word.chars()
                .map(|letter| map.get(&letter).unwrap().clone())
                .fold(0u32, |acc, x| acc * 10 + x as u32)
        })
        .sum::<u32>()
}

impl Puzzle {
    fn from(puzzle: &str) -> Puzzle {
        let mut chars = puzzle.chars()
            .filter(|c| c.is_alphabetic())
            .collect::<Vec<char>>();
        chars.sort();
        chars.dedup();
        let mut parts = puzzle.split("==");
        let left = parts.next()
            .unwrap()
            .split("+")
            .into_iter()
            .map(|s| String::from(s.trim()))
            .collect::<Vec<String>>();

        let right = parts.next()
            .unwrap()
            .split("+")
            .into_iter()
            .map(|s| String::from(s.trim()))
            .collect::<Vec<String>>();

        let leading_letters = left.iter()
            .chain(right.iter())
            .map(|word| word.chars().next().unwrap())
            .collect::<Vec<char>>();

        Puzzle {
            unique_chars: chars,
            left: left,
            right: right,
            leading_chars: leading_letters,
        }
    }

    fn equal_with_map(&self, map: &Mapping) -> bool {
        part_value(&self.left, map) == part_value(&self.right, map)
    }

    pub fn solve(&self) -> Option<Mapping> {
        self.solveing(vec![], self.unique_chars.len() as u8)
    }

    fn solveing(&self, part: Vec<u8>, iter: u8) -> Option<Mapping> {
        if iter == 0 {
            let map =
                self.unique_chars.iter().cloned().zip(part.iter().cloned()).collect::<Mapping>();
            let valid = self.leading_chars.iter().all(|c| map.get(&c).unwrap().clone() != 0u8);
            if !valid {
                return None;
            }

            if self.equal_with_map(&map) {
                return Some(map);
            }
        } else {
            for x in 0..10 {
                if part.contains(&x) {
                    continue;
                }
                let mut next_part = part.clone();
                next_part.push(x);
                let solution = self.solveing(next_part, iter - 1);
                if solution.is_some() {
                    return solution;
                }
            }
        }
        None
    }
}


pub fn solve(puzzle: &str) -> Option<Mapping> {
    Puzzle::from(puzzle).solve()
}
