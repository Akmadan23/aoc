use std::fs;
use std::str::FromStr;
use std::num::ParseIntError;
use regex::Regex;

struct Data(Vec<u32>);

impl FromStr for Data {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
        let mut v = vec![];

        for (_, [a, b]) in re.captures_iter(s).map(|c| c.extract()) {
            v.push(a.parse::<u32>()? * b.parse::<u32>()?);
        }

        Ok(Self(v))
    }
}

pub fn main() {
    let data: Vec<(Data, Data)> = fs::read_to_string("data/2024/03.txt")
        .unwrap()
        .split("don't()")
        .map(|s| s
            .split_once("do()")
            .unwrap_or((s, "")))
        .filter_map(|(a, b)| Some((a.parse().ok()?, b.parse().ok()?)))
        .collect();

    let (mut result1, mut result2) = (0, 0);

    for (i, (d1, d2)) in data.iter().enumerate() {
        let sum1 = d1.0.iter().sum::<u32>();
        let sum2 = d2.0.iter().sum::<u32>();

        result1 += sum1 + sum2;

        if i == 0 {
            result2 = result1;
        } else {
            result2 += sum2;
        }
    }

    println!("Result 1: {}", result1); // Result 1: 157621318
    println!("Result 2: {}", result2); // Result 2: 79845780
}
