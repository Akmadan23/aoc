use std::{
    ops::{ Deref, DerefMut },
    str::FromStr,
    num::ParseIntError,
    convert::Infallible,
};

struct Stack(Vec<Vec<char>>);

impl Deref for Stack {
    type Target = Vec<Vec<char>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Stack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromStr for Stack {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stacks = vec![vec![]; 9];

        for line in s.lines().rev().skip(1) {
            for (index, char) in format!("{line: >35}").chars().skip(1).step_by(4).enumerate() {
                if ('A'..='Z').contains(&char) {
                    if let Some(s) = stacks.get_mut(index) {
                        s.push(char);
                    }
                }
            }
        }

        Ok(Self(stacks))
    }
}

struct Move {
    num: usize,
    from: usize,
    to: usize,
}

impl FromStr for Move {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [mut num, mut from, mut to] = [0; 3];

        for (index, word) in s.split(" ").enumerate() {
            if index == 0 {
                if word != "move" {
                    // return ParseIntError
                    let _: u8 = "error".parse()?;
                }
            }

            match index {
                1 => num = word.parse()?,
                3 => from = word.parse::<usize>()? - 1,
                5 => to = word.parse::<usize>()? - 1,
                _ => continue
            }
        }

        Ok(Self { num, from, to })
    }
}

pub fn main() {
    let moves: Vec<Move> = aoc::file_to_vec("data/2022/05.txt", "\n").unwrap();
    let stacks: Vec<Stack> = aoc::file_to_vec("data/2022/05.txt", "\n\n").unwrap();

    let mut stacks1 = stacks[0].clone();
    let mut stacks2 = stacks[0].clone();
    let mut result1 = String::new();
    let mut result2 = String::new();

    for m in moves {
        let mut tmp = vec![];

        for _ in 0..m.num {
            if let Some(value) = stacks1[m.from].pop() {
                stacks1[m.to].push(value);
            }

            if let Some(value) = stacks2[m.from].pop() {
                tmp.push(value);
            }
        }

        for value in tmp.into_iter().rev() {
            stacks2[m.to].push(value);
        }
    }

    for i in 0..stacks[0].len() {
        if let Some(value) = stacks1[i].pop() {
            result1.push(value)
        }

        if let Some(value) = stacks2[i].pop() {
            result2.push(value)
        }
    }

    println!("Result 1: {}", result1); // Result 1: NTWZZWHFV
    println!("Result 2: {}", result2); // Result 2: BRZGFVBTJ
}
