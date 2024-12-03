use std::{str::FromStr, num::ParseIntError};

#[derive(Clone)]
struct Report(Vec<u8>);

impl Report {
    fn take(&self, i: usize) -> Self {
        let mut new_vec = self.0.clone();
        new_vec.remove(i);
        Self(new_vec)
    }
}

impl FromStr for Report {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s
            .split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect()))
    }
}

fn is_safe(r: &&mut Report) -> bool {
    let asc = r.0[0] < r.0[1];

    for w in r.0.windows(2) {
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

fn is_safe_rec(r: &&mut Report) -> bool {
    if is_safe(r) {
        return true;
    }

    for i in 0..r.0.len() {
        if is_safe(&&mut r.take(i)) {
            return true;
        }
    }

    false
}

pub fn main() {
    let mut reports: Vec<Report> = aoc::read_from_file("data/2024/02.txt", "\n").unwrap();

    let result1 = reports
        .iter_mut()
        .filter(is_safe)
        .count();

    let result2 = reports
        .iter_mut()
        .filter(is_safe_rec)
        .count();

    println!("Result 1: {}", result1); // Result 1: 279
    println!("Result 2: {}", result2); // Result 2: 342 < x < 407, !386
}
