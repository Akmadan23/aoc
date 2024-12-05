use std::{
    str::FromStr,
    convert::Infallible,
};

struct Pair {
    min1: u8,
    max1: u8,
    min2: u8,
    max2: u8,
}

impl FromStr for Pair {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x: Vec<u8> = s
            .split([',', '-'])
            .filter_map(|i| i.parse().ok())
            .collect();

        Ok(Self {
            min1: x[0],
            max1: x[1],
            min2: x[2],
            max2: x[3],
        })
    }
}

pub fn main() {
    let pairs: Vec<Pair> = aoc::file_to_vec("data/2022/04.txt", "\n").unwrap();
    let [mut result1, mut result2] = [0; 2];

    for p in pairs {
        if (p.min1 >= p.min2 && p.max1 <= p.max2)
        || (p.min2 >= p.min1 && p.max2 <= p.max1) {
            result1 += 1;
        }

        if (p.min1 >= p.min2 && p.min1 <= p.max2)
        || (p.max1 >= p.min2 && p.max1 <= p.max2)
        || (p.min2 >= p.min1 && p.min2 <= p.max1)
        || (p.max2 >= p.min1 && p.max2 <= p.max1) {
            result2 += 1;
        }
    }

    println!("Result 1: {}", result1); // Result 1: 8105
    println!("Result 2: {}", result2); // Result 2: 2363
}
