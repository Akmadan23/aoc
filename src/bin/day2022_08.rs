use std::{
    str::FromStr,
    num::ParseIntError
};

#[derive(Debug)]
struct Tree {
    height: i8,
    visible: bool
}

impl FromStr for Tree {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            height: s.parse()?,
            visible: false
        })
    }
}

pub fn main() {
    let mut rows: Vec<Vec<Tree>> = aoc::file_to_matrix("data/2022/08.txt", "\n", "").unwrap();
    let mut scores = Vec::new();
    let mut result1 = 0;

    // Create a single range to iterate the array in both directions at the same time
    let len = rows.len();
    let range = (0..len).zip((0..len).rev());

    for (a, x) in range.clone() {
        let mut max = [-1; 4];

        for (b, y) in range.clone() {
            for (idx, (i, j)) in [(a, b), (b, a), (x, y), (y, x)].into_iter().enumerate() {
                let tree = &mut rows[i][j];

                if tree.height > max[idx] {
                    max[idx] = tree.height;

                    if !tree.visible {
                        tree.visible = true;
                        result1 += 1;
                    }
                }
            }

            let cur_height = rows[a][b].height;

            let mut ranges: [(&mut dyn Iterator<Item = usize>, Option<usize>, Option<usize>, usize); 4] = [
                (&mut ((a + 1)..len), None,     Some(b),    0),
                (&mut ((b + 1)..len), Some(a),  None,       0),
                (&mut ((0..a).rev()), None,     Some(b),    0),
                (&mut ((0..b).rev()), Some(a),  None,       0),
            ];

            for (range, i, j, count) in &mut ranges {
                for k in range {
                    let next_height = rows[i.unwrap_or(k)][j.unwrap_or(k)].height;

                    if next_height <= cur_height {
                        *count += 1;
                    }

                    if next_height >= cur_height {
                        break
                    }
                }
            }

            scores.push(ranges
                .iter()
                .fold(1, |m, (.., c)| m * c));
        }
    }

    let result2 = scores
        .iter()
        .max()
        .unwrap();

    println!("Result 1: {}", result1); // Result 1: 1533
    println!("Result 2: {}", result2); // Result 2: 345744
}
