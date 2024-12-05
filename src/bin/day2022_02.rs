pub fn main() {
    let games: Vec<String> = aoc::file_to_vec("data/2022/02.txt", "\n").unwrap();
    let [mut result1, mut result2] = [0; 2];

    for g in games {
        let (score1, score2) = match g.split_once(" ").unwrap() {
            ("A", "X") => (4, 3),
            ("A", "Y") => (8, 4),
            ("A", "Z") => (3, 8),
            ("B", "X") => (1, 1),
            ("B", "Y") => (5, 5),
            ("B", "Z") => (9, 9),
            ("C", "X") => (7, 2),
            ("C", "Y") => (2, 6),
            ("C", "Z") => (6, 7),
            _ => (0, 0)
        };

        result1 += score1;
        result2 += score2;
    }

    println!("Result 1: {}", result1); // Result 1: 9651
    println!("Result 2: {}", result2); // Result 2: 10560
}
