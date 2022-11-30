pub fn main() {
    let lines: Vec<String> = aoc::read_from_file("data/2020/03.txt", "\n").unwrap();
    let steps = [3, 1, 5, 7, 1];
    let [mut indexes, mut results] = [[0; 5]; 2];

    for i in 0..lines.len() {
        let chars: Vec<char> = lines[i].chars().collect();
        let len = chars.len();

        for j in 0..5 {
            if indexes[j] >= len {
                indexes[j] -= len;
            }

            if j != 4 || i % 2 == 0 {
                if chars[indexes[j]] == '#' {
                    results[j] += 1;
                }

                indexes[j] += steps[j];
            }
        }
    }

    println!("Result 1: {}", results[0]);   // Result 1: 203
    println!("Result 2: {}", results        // Result 2: 3316272960
        .iter()
        .fold(1, |m, i| m * i));
}
