use std::{
    str::FromStr,
    ops::{ Deref, DerefMut },
};

enum Direction {
    North,
    South,
    West,
    East,
}

struct CharVec(Vec<char>);

impl Deref for CharVec {
    type Target = Vec<char>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CharVec {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromStr for CharVec {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.chars().collect()))
    }
}

fn scan(lines: &mut Vec<CharVec>, from: Direction, i: usize, j: usize, count: usize) -> usize {
    let cur_char = lines[i][j];

    if cur_char == 'S' {
        let [north, south, west, east] = [
            match i == 0 {
                false => "|/71Ff".contains(lines[i - 1][j]),
                true => false,
            },
            match lines.get(i + 1) {
                Some(line) => "|/JjLl".contains(line[j]),
                None => false,
            },
            match j == 0 {
                false => "-+Jj71".contains(lines[i][j - 1]),
                true => false,
            },
            match lines[i].get(j + 1) {
                Some(&char) => "-+llFf".contains(char),
                None => false,
            },
        ];

        if count > 0 {
            lines[i][j] = match [north, south, west, east] {
                [true, true, _, _] => '/',
                [true, _, true, _] => 'j',
                [true, _, _, true] => 'l',
                [_, true, _, true] => 'f',
                [_, _, true, true] => '+',
                [_, true, true, _] => '1',
                _ => panic!(),
            };

            return count;
        } else {
            let (new_from, new_i, new_j) = match [north, south, west, east] {
                [true, ..] => (Direction::South, i - 1, j),
                [_, true, ..] => (Direction::North, i + 1, j),
                [.., true, _] => (Direction::East, i, j - 1),
                [.., true] => (Direction::West, i, j + 1),
                _ => panic!(),
            };

            return scan(lines, new_from, new_i, new_j, count + 1);
        }
    }

    use Direction::*;
    let new_from = match from {
        North => match cur_char {
            '|' => North,
            'L' => West,
            'J' => East,
            _ => panic!()
        },

        South => match cur_char {
            '|' => South,
            'F' => West,
            '7' => East,
            _ => panic!()
        },

        West => match cur_char {
            '-' => West,
            'J' => South,
            '7' => North,
            _ => panic!()
        },

        East => match cur_char {
            '-' => East,
            'L' => South,
            'F' => North,
            _ => panic!()
        },
    };

    let (new_i, new_j) = match new_from {
        North => (i + 1, j),
        South => (i - 1, j),
        West => (i, j + 1),
        East => (i, j - 1),
    };

    // remap chars to keep track of the loop
    lines[i][j] = match cur_char {
        'F' => 'f',
        'J' => 'j',
        'L' => 'l',
        '7' => '1',
        '|' => '/',
        '-' => '+',
        _ => panic!(),
    };

    return scan(lines, new_from, new_i, new_j, count + 1);
}

pub fn main() {
    let mut lines: Vec<CharVec> = aoc::read_from_file("data/2023/10.txt", "\n").unwrap();
    let mut start = (0, 0);

    for (i, line) in lines.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            if *char == 'S' {
                start = (i, j);
                break;
            }
        }
    }

    let result1 = scan(&mut lines, Direction::North, start.0, start.1, 0) / 2;
    let mut result2 = 0;

    for line in lines.iter_mut() {
        let mut count = 0;
        let mut last = '.';

        for char in line.iter_mut() {
            match (last, char) {
                (_, 'f') => last = 'f',
                (_, 'l') => last = 'l',
                (_, '/') | ('f', 'j') | ('l', '1') => count += 1,
                (_, '+') | ('f', '1') | ('l', 'j') => (),
                _ => if count % 2 == 1 {
                    result2 += 1;
                }
            }
        }
    }

    println!("Result 1: {}", result1); // Result 1: 6714
    println!("Result 2: {}", result2); // Result 2: 429
}
