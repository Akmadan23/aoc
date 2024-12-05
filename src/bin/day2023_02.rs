use std::{str::FromStr, num::ParseIntError};

#[derive(Debug, Default)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

impl FromStr for Set {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut set = Self::default();

        for ext in s.split(',') {
            let (nr, color) = ext.trim().split_once(' ').unwrap();
            *match color {
                "red" => &mut set.red,
                "green" => &mut set.green,
                "blue" => &mut set.blue,
                _ => panic!("This should never happen"),
            } = nr.parse()?;
        }

        Ok(set)
    }
}

struct Game {
    id: u32,
    sets: Vec<Set>,
}

impl FromStr for Game {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, body) = s.split_once(':').unwrap();

        Ok(Self {
            id: id
                .split_once(' ')
                .unwrap().1
                .parse()?,
            sets: body
                .split(';')
                .filter_map(|set| Some(Set::from_str(set).ok()?))
                .collect()
        })
    }
}

pub fn main() {
    let games: Vec<Game> = aoc::file_to_vec("data/2023/02.txt", "\n").unwrap();
    let [mut result1, mut result2] = [0; 2];

    for game in &games {
        let mut possible = true;
        let mut min = Set::default();

        for set in &game.sets {
            if possible && set.red > 12 || set.green > 13 || set.blue > 14 {
                possible = false;
            }

            if set.red > min.red {
                min.red = set.red;
            }

            if set.green > min.green {
                min.green = set.green;
            }

            if set.blue > min.blue {
                min.blue = set.blue;
            }
        }

        if possible {
            result1 += game.id;
        }

        result2 += min.red * min.green * min.blue;
    }

    println!("Result 1: {}", result1); // Result 1: 2545
    println!("Result 2: {}", result2); // Result 2: 78111
}
