const SIZE: usize = 5;

#[derive(Default)]
struct Board([[(u8, bool); SIZE]; SIZE]);

impl Board {
    fn set(&mut self, col: usize, row: usize, value: u8) {
        self.0[col][row].0 = value
    }

    fn get(&self, col: usize, row: usize) -> u8 {
        self.0[col][row].0
    }

    fn check(&mut self, col: usize, row: usize) {
        self.0[col][row].1 = true
    }

    fn is_checked(&self, col: usize, row: usize) -> bool {
        self.0[col][row].1
    }

    fn row_win(&self, row: usize) -> bool {
        for i in 0..SIZE {
            if !self.0[i][row].1 {
                return false
            }
        }

        return true;
    }

    fn col_win(&self, col: usize) -> bool {
        for i in 0..SIZE {
            if !self.0[col][i].1 {
                return false
            }
        }

        return true;
    }
}

pub fn main() {
    let lines: Vec<String> = lib::read_from_file("data/4.txt", "\n").unwrap();

    let numbers: Vec<u8> = lines[0].split(",")
        .map(|i| i.parse().unwrap())
        .collect();

    let mut boards = vec![];
    let [mut result, mut first_result] = [0; 2];
    let mut found = false;

    for i in (1..lines.len()).step_by(SIZE) {
        let mut board_line: Vec<u8>;
        let mut board = Board::default();

        for j in 0..SIZE {
            board_line = lines[i+j]
                .split(" ")
                .filter_map(|num| num.parse().ok())
                .collect();

            for k in 0..SIZE {
                board.set(j, k, board_line[k]);
            }
        }

        boards.push(board);
    }

    for n in numbers {
        for board in boards.iter_mut() {
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
                                sum += boards[i].get(col, row) as i32
                            }
                        }
                    }

                    boards.remove(i);
                    result = sum * n as i32;

                    if !found {
                        first_result = result;
                        found = true;
                    }
                }
            }
        }
    }

    println!("Result 1: {}", first_result); // Result 1: 2745
    println!("Result 2: {}", result);       // Result 1: 6594
}
