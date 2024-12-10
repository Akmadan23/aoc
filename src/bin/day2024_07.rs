use std::{
    str::FromStr,
    num::ParseIntError,
};

struct Data {
    total: usize,
    items: Vec<usize>,
}

impl FromStr for Data {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once(':').unwrap();
        Ok(Self {
            total: a.parse()?,
            items: b
                .split(" ")
                .filter_map(|n| n.parse().ok())
                .collect()
        })
    }
}

pub fn main() {
    let data: Vec<Data> = aoc::file_to_vec("data/2024/07.txt", "\n").unwrap();
    let (mut result1, mut result2) = (0, 0);

    for d in &data {
        let [mut found1, mut found2] = [false; 2];
        for ops in 0..3_usize.pow(d.items.len() as u32) {
            let mut acc = d.items[0];
            let mut flag2 = false;

            for (index, item) in d.items.iter().skip(1).enumerate() {
                match (ops / 3_usize.pow(index as u32)) % 3 {
                    0 => acc += item,
                    1 => acc *= item,
                    2 => {
                        // this flag filters results valid for first or second task
                        flag2 = true;
                        acc = format!("{acc}{item}").parse().unwrap();
                    }
                    _ => (),
                }
            }

            if d.total == acc {
                if !found2 {
                    result2 += acc;
                    found2 = true;
                }

                if !flag2 && !found1 {
                    result1 += acc;
                    found1 = true;
                }

                if found1 && found2 {
                    break;
                }
            }
        }
    }

    println!("Result 1: {}", result1); // Result 1: 3351424677624
    println!("Result 2: {}", result2); // Result 2: 204976636995111
}
