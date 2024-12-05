use std::collections::HashMap;

type MovesMap<'a> = HashMap<&'a str, (&'a str, &'a str)>;

fn get_steps<'a>(moves: &MovesMap<'a>, directions: &Vec<char>, mut next: &'a str, exit_condition: fn(&str) -> bool) -> usize {
    let mut steps = 0;
    while let Some(node) = moves.get(&next) {
        if exit_condition(next) {
            break;
        }

        next = match directions[steps % directions.len()] {
            'L' => node.0,
            'R' => node.1,
            _ => panic!(),
        };

        steps += 1;
    }

    steps
}

fn gcd(mut max: usize, mut min: usize) -> usize {
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}					

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

pub fn main() {
    let lines: Vec<String> = aoc::file_to_vec("data/2023/08.txt", "\n\n").unwrap();

    let directions: Vec<char> = lines[0]
        .chars()
        .collect();

    let moves: MovesMap = lines[1]
        .lines()
        .map(|s| {
            let x: Vec<&str> = s
                .split(" ")
                .map(|s| s.trim_matches(|c| "(),".contains(c)))
                .collect();

            (x[0], (x[2], x[3]))
        })
        .collect();

    let result1 = get_steps(&moves, &directions, "AAA", |x| x == "ZZZ");
    let result2 = moves
        .keys()
        .filter(|s| s.ends_with('A'))
        .map(|s| get_steps(&moves, &directions, s, |x| x.ends_with('Z')))
        .fold(1, |acc, x| lcm(acc, x));

    println!("Result 1: {}", result1); // Result 1: 12599
    println!("Result 2: {}", result2); // Result 2: 8245452805243
}
