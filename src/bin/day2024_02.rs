fn is_safe(v: &&Vec<u8>) -> bool {
    let asc = v[0] < v[1];

    for w in v.windows(2) {
        let (a, b) = (w[0], w[1]);

        if a == b
            || (asc && a > b)
            || (!asc && a < b)
            || !matches!(a.abs_diff(b), 1 | 2 | 3)
        {
            return false;
        }
    }

    true
}

fn is_safe_rec(v: &&Vec<u8>) -> bool {
    if is_safe(v) {
        return true;
    }

    for i in 0..v.len() {
        let mut new_v = (*v).clone();
        new_v.remove(i);

        if is_safe(&&new_v) {
            return true;
        }
    }

    false
}

pub fn main() {
    let reports: Vec<Vec<u8>> = aoc::file_to_matrix("data/2024/02.txt", "\n", " ").unwrap();

    let result1 = reports
        .iter()
        .filter(is_safe)
        .count();

    let result2 = reports
        .iter()
        .filter(is_safe_rec)
        .count();

    println!("Result 1: {}", result1); // Result 1: 279
    println!("Result 2: {}", result2); // Result 2: 343
}
