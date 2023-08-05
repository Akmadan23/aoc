pub fn main() {
    let v: Vec<u32> = aoc::read_from_file("data/2021/07.txt", ",").unwrap();
    let [mut result1, mut result2] = [u32::MAX; 2];

    let max = *v
        .iter()
        .max()
        .unwrap();

    for pos in 0..=max {
        let [mut current1, mut current2] = [0; 2];

        for elem in &v {
            let n = elem.abs_diff(pos);
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
