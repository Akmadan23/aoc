pub fn main() {
    let food: Vec<Vec<u32>> = aoc::file_to_matrix("data/2022/01.txt", "\n\n", "\n").unwrap();
    let mut results = [0; 3];

    'i: for i in 0..results.len() {
        for f in &food {
            let sum: u32 = f.iter().sum();

            if sum > results[i] {
                for r in &results {
                    if sum == *r {
                        continue 'i
                    }
                }

                results[i] = sum;
            }
        }
    }

    println!("Result 1: {}", results[0]);                   // Result 1: 69501
    println!("Result 2: {}", results.iter().sum::<u32>());  // Result 2: 202346
}
