use std::{
    ops::Deref,
    str::FromStr,
    convert::Infallible,
};

struct RuckSack(Vec<u16>);

impl Deref for RuckSack {
    type Target = Vec<u16>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for RuckSack {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s
            .chars()
            .map(|i| match i {
                'a'..='z' => i as u16 - 96,
                'A'..='Z' => i as u16 - 38,
                _ => 0
            })
            .collect()))
    }
}

pub fn main() {
    let rucksacks: Vec<RuckSack> = aoc::file_to_vec("data/2022/03.txt", "\n").unwrap();
    let [mut result1, mut result2] = [0; 2];

    'c: for c in rucksacks.chunks(3) {
        'r: for r in c {
            let half = r.len() / 2;

            for i in &r[..half] {
                for j in &r[half..] {
                    if i == j {
                        result1 += i;
                        continue 'r
                    }
                }
            }
        }

        for i in &*c[0] {
            for j in &*c[1] {
                if i == j {
                    for k in &*c[2] {
                        if j == k {
                            result2 += i;
                            continue 'c
                        }
                    }
                }
            }
        }
    }

    println!("Result 1: {}", result1); // Result 1: 8105
    println!("Result 2: {}", result2); // Result 2: 2363
}
