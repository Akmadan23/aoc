use std::collections::HashSet;

pub fn main() {
    let stream: Vec<String> = aoc::read_from_file("data/2022/06.txt", "").unwrap();

    let result = |len| {
        for (index, window) in stream.windows(len).enumerate() {
            let set: HashSet<&String> = window
                .iter()
                .collect();

            if set.len() == len {
                return index + len;
            }
        }

        panic!("Incorrect input")
    };

    println!("Result 1: {}", result(4));    // Result 1: 1640
    println!("Result 2: {}", result(14));   // Result 2: 3613
}
