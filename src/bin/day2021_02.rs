use std::{
    str::FromStr,
    num::ParseIntError
};

enum Direction {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl FromStr for Direction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, value): (&str, u32) = match s.split_once(" ") {
            Some((dir, val)) => (dir, val.parse()?),
            None => ("forward", 0)
        };

        Ok(match direction {
            "forward" => Self::Forward(value),
            "down" => Self::Down(value),
            "up" => Self::Up(value),
            _ => panic!("Unable to parse direction.")
        })
    }
}

pub fn main() {
    let directions: Vec<Direction> = aoc::read_from_file("data/2021/02.txt", "\n").unwrap();

    let [
        mut pos,
        mut aim,
        mut depth_old,
        mut depth_new,
    ] = [0; 4];

    for d in directions {
        use Direction::*;

        match d {
            Forward(val) => {
                pos += val;
                depth_new += val * aim;
            },

            Down(val) => {
                depth_old += val;
                aim += val;
            },

            Up(val) => {
                depth_old -= val;
                aim -= val;
            }
        }
    }

    println!("Result 1: {}", pos * depth_old);  // Result 1: 1698735
    println!("Result 2: {}", pos * depth_new);  // Result 2: 1594785890
}
