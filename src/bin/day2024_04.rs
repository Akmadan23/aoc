const SIZE: usize = 140;

pub fn main() {
    let c: Vec<Vec<char>> = aoc::file_to_matrix("data/2024/04.txt", "\n", "").unwrap();
    let (mut result1, mut result2) = (0, 0);

    for i in 0..SIZE {
        for j in 0..SIZE {
            let mut words = vec![];

            if i < SIZE - 3 {
                words.push([c[i][j], c[i+1][j], c[i+2][j], c[i+3][j]]);
            }

            if j < SIZE - 3 {
                words.push([c[i][j], c[i][j+1], c[i][j+2], c[i][j+3]]);
            }

            if i < SIZE - 3 && j < SIZE - 3 {
                words.push([c[i][j], c[i+1][j+1], c[i+2][j+2], c[i+3][j+3]]);
                words.push([c[i][j+3], c[i+1][j+2], c[i+2][j+1], c[i+3][j]]);
            }

            for w in &words {
                if matches!(w, ['X', 'M', 'A', 'S'] | ['S', 'A', 'M', 'X']) {
                    result1 += 1;
                }
            }

            if i < SIZE - 2 && j < SIZE - 2 {
                if c[i+1][j+1] == 'A' && matches!(
                    (c[i][j], c[i][j+2], c[i+2][j], c[i+2][j+2]),
                    ('M', 'S', 'M', 'S') | ('M', 'M', 'S', 'S') | ('S', 'M', 'S', 'M') | ('S', 'S', 'M', 'M')
                ) {
                    result2 += 1;
                }
            }
        }
    }

    println!("Result 1: {}", result1); // Result 1: 2547
    println!("Result 2: {}", result2); // Result 2: 1939
}
