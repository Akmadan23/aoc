pub fn main() {
    let crabs: Vec<u32> = aoc::read_from_file("data/2021/07.txt", ",").unwrap();
    let [mut result1, mut result2] = [u32::MAX; 2];

    let max = *crabs
        .iter()
        .max()
        .unwrap();

    for pos in 0..=max {
        let [mut current1, mut current2] = [0; 2];

        for c in &crabs {
            let n = c.abs_diff(pos);
            current1 += n;
            current2 += n * (n + 1) / 2;
        }

        if current1 < result1 {
            result1 = current1
        }

        if current2 < result2 {
            result2 = current2
        }
    }

    println!("Result 1: {}", result1); // Result 1: 341558
    println!("Result 2: {}", result2); // Result 2: 93214037
}
