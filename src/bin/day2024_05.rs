fn check_and_fix_order(v: &mut Vec<u16>, rules: &Vec<(u16, u16)>) -> bool {
    let len = v.len() - 1;
    let mut already_ordered = true;
    let mut ordered = false;

    while !ordered {
        ordered = true;

        for i in 0..len {
            for j in (i+1)..=len {
                if rules.contains(&(v[j], v[i])) {
                    let t = v[i];
                    v[i] = v[j];
                    v[j] = t;
                    ordered = false;
                    already_ordered = false;
                }
            }
        }
    }

    already_ordered
}

pub fn main() {
    let data: Vec<String> = aoc::file_to_vec("data/2024/05.txt", "\n\n").unwrap();
    let (mut result1, mut result2) = (0, 0);

    let rules: Vec<(u16, u16)> = data[0]
        .split("\n")
        .map(|l| l
            .split('|')
            .filter_map(|n| n.parse().ok())
            .collect::<Vec<_>>())
        .map(|x| (x[0], x[1]))
        .collect();

    let updates: Vec<Vec<u16>> = data[1]
        .split("\n")
        .map(|l| l
            .split(',')
            .filter_map(|n| n.parse().ok())
            .collect())
        .collect();

    for mut u in updates {
        if check_and_fix_order(&mut u, &rules) {
            result1 += u[u.len()/2]; 
        } else {
            result2 += u[u.len()/2];
        }
    }

    println!("Result 1: {}", result1); // Result 1: 5329
    println!("Result 2: {}", result2); // Result 2: 5833
}
