use std::{
    str::FromStr,
    num::ParseIntError,
};

#[derive(Default)]
struct Password {
    min: usize,
    max: usize,
    char: char,
    psw: Vec<char>
}

impl FromStr for Password {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((part1, psw)) = s.split_once(": ") {
            if let Some((range, char)) = part1.split_once(" ") {
                if let Some((first, second)) = range.split_once("-") {
                    return Ok(Self {
                        min: first.parse()?,
                        max: second.parse()?,
                        char: char.chars().next().unwrap(),
                        psw: psw.chars().collect()
                    })
                }
            }
        }

        Ok(Self::default())
    }
}

pub fn main() {
    let psw: Vec<Password> = aoc::file_to_vec("data/2020/02.txt", "\n").unwrap();
    let [mut result1, mut result2] = [0; 2];

    for p in &psw {
        let count = p.psw
            .iter()
            .filter(|&&x| x == p.char)
            .count();

        if count >= p.min && count <= p.max {
            result1 += 1;
        }

        if (p.psw[p.min - 1] == p.char && p.psw[p.max - 1] != p.char)
        || (p.psw[p.min - 1] != p.char && p.psw[p.max - 1] == p.char) {
            result2 += 1;
        }
    }

    println!("Result 1: {}", result1); // Result 1: 645
    println!("Result 2: {}", result2); // Result 2: 737
}
