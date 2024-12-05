const LITERALS: [(u32, &str); 9] = [
    (1, "one"),
    (2, "two"),
    (3, "three"),
    (4, "four"),
    (5, "five"),
    (6, "six"),
    (7, "seven"),
    (8, "eight"),
    (9, "nine"),
];

fn check_literal(s: &str) -> Option<u32> {
    for l in &LITERALS {
        if s.contains(l.1) {
            return Some(l.0)
        }
    }

    None
}

fn evaluate_digits(char: char, substr: &str, digit_v1: &mut u32, digit_v2: &mut u32) -> bool {
    if substr.len() >= 3 && *digit_v2 == 0 {
        if let Some(x) = check_literal(&substr) {
            *digit_v2 = x;
        }
    }

    if let Some(x) = char.to_digit(10) {
        if *digit_v1 == 0 {
            *digit_v1 = x;
        }

        if *digit_v2 == 0 {
            *digit_v2 = x;
        }
    }

    return *digit_v1 != 0 && *digit_v2 != 0
}

pub fn main() {
    let lines: Vec<String> = aoc::file_to_vec("data/2023/01.txt", "\n").unwrap();
    let [mut result1, mut result2] = [0; 2];

    for line in &lines {
        let [mut digit1_v1, mut digit1_v2] = [0; 2];
        let mut substr = String::new();

        for char in line.chars() {
            substr.push(char);
            if evaluate_digits(char, &substr, &mut digit1_v1, &mut digit1_v2) {
                break;
            }
        }

        let [mut digit2_v1, mut digit2_v2] = [0; 2];
        let mut substr = String::new();

        for char in line.chars().rev() {
            substr.insert(0, char);
            if evaluate_digits(char, &substr, &mut digit2_v1, &mut digit2_v2) {
                break;
            }
        }

        result1 += digit1_v1 * 10 + digit2_v1;
        result2 += digit1_v2 * 10 + digit2_v2;
    }

    println!("Result 1: {}", result1); // Result 1: 55017
    println!("Result 2: {}", result2); // Result 2: 53539
}
