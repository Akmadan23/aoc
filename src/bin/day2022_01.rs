use std::{
    ops::Deref,
    str::FromStr,
    convert::Infallible,
};

struct Food(Vec<u32>);

impl Deref for Food {
    type Target = Vec<u32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Food {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s
            .split("\n")
            .filter_map(|i| i.parse().ok())
            .collect()))
    }
}

pub fn main() {
    let food: Vec<Food> = aoc::read_from_file("data/2022/01.txt", "\n\n").unwrap();
    let mut results = [0; 3];

    'i: for i in 0..results.len() {
        for f in &food {
            let sum: u32 = f.iter().sum();

            if sum > results[i] {
                for r in &results {
                    if sum == *r {
                        continue 'i
                    }
                }

                results[i] = sum;
            }
        }
    }

    println!("Result 1: {}", results[0]);                   // Result 1: 69501
    println!("Result 2: {}", results.iter().sum::<u32>());  // Result 2: 202346
}
