pub fn main() {
    let depths: Vec<i32> = aoc::read_from_file("data/2021/01.txt", "\n").unwrap();
    let [mut result1, mut result2] = [0; 2];

    for i in 1..depths.len() {
        if depths[i] > depths[i - 1] {
            result1 += 1;
        }

        if i > 2 && depths[i] > depths[i - 3] {
            result2 += 1;
        }
    }

    println!("Result 1: {}", result1); // Result 1: 1521
    println!("Result 2: {}", result2); // Result 2: 1543
}
