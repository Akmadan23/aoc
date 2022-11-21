pub fn main() {
    let entries: Vec<u32> = aoc::read_from_file("data/2020/01.txt", "\n").unwrap();
    let [mut result1, mut result2] = [0; 2];

    'a: for a in &entries {
        for b in &entries {
            if a + b == 2020 {
                result1 = a * b;
            }

            if result2 == 0 {
                for c in &entries {
                    if a + b + c == 2020 {
                        result2 = a * b * c;
                    }
                }
            }

            if result1 != 0 && result2 != 0 {
                break 'a;
            }
        }
    }

    println!("Result 1: {}", result1); // Result 1: 793524
    println!("Result 2: {}", result2); // Result 2: 61515678
}
