use std::{
    str::FromStr,
    num::ParseIntError
};

const SIZE: usize = 5;

#[derive(Clone)]
struct Board(Vec<Vec<(u16, bool)>>);

impl Board {
    fn get(&self, col: usize, row: usize) -> u16 {
        self.0[col][row].0
    }

    fn check(&mut self, col: usize, row: usize) {
        self.0[col][row].1 = true
    }

    fn is_checked(&self, col: usize, row: usize) -> bool {
        self.0[col][row].1
    }

    fn row_win(&self, row: usize) -> bool {
        for col in 0..SIZE {
            if !self.0[col][row].1 {
                return false;
            }
        }

        return true;
    }

    fn col_win(&self, col: usize) -> bool {
        for row in 0..SIZE {
            if !self.0[col][row].1 {
                return false;
            }
        }

        return true;
    }
}

impl FromStr for Board {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s
            .split("\n")
            .map(|i| i
                .split(" ")
                .filter_map(|j| j.parse().ok())
                .map(|j| (j, false))
                .collect::<Vec<_>>())
            .collect()))
    }
}

pub fn main() {
    let numbers: Vec<u16> = aoc::read_from_file("data/2021/04.txt", ",").unwrap();
    let mut boards: Vec<Board> = aoc::read_from_file("data/2021/04.txt", "\n\n").unwrap()[1..].to_vec();
    let [mut result1, mut result2] = [0; 2]; // first and last
    let mut first = false;

    for n in numbers {
        for board in &mut boards {
            for col in 0..SIZE {
                for row in 0..SIZE {
                    if board.get(col, row) == n {
                        board.check(col, row);
                    }
                }
            }
        }

        for i in 0..boards.len() {
            for j in 0..SIZE {
                if i >= boards.len() {
                    break
                }

                if boards[i].row_win(j) || boards[i].col_win(j) {
                    let mut sum = 0;

                    for col in 0..SIZE {
                        for row in 0..SIZE {
                            if !boards[i].is_checked(col, row) {
                                sum += boards[i].get(col, row)
                            }
                        }
                    }

                    boards.remove(i);
                    result2 = sum * n;

                    if !first {
                        result1 = result2;
                        first = true;
                    }
                }
            }
        }
    }

    println!("Result 1: {}", result1); // Result 1: 2745
    println!("Result 2: {}", result2); // Result 1: 6594
}
