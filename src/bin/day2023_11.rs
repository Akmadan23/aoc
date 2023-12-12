pub fn main() {
    let mut lines: Vec<String> = aoc::read_from_file("data/2023/11.txt", "\n").unwrap();
    let mut galaxies1 = vec![];
    let mut columns_to_expand = vec![];
    let mut lines_to_expand = vec![];
    let [mut result1, mut result2] = [0; 2];

    for i in 0..lines[0].len() {
        lines
            .iter()
            .filter(|l| l
                .chars()
                .nth(i)
                .unwrap()
                .eq(&'#'))
            .count()
            .eq(&0)
            .then(|| columns_to_expand.push(i));
    }

    for (i, line) in lines.iter_mut().enumerate() {
        for (j, char) in line.chars().enumerate() {
            if char == '#' {
                galaxies1.push((i, j));
            }
        }

        line
            .chars()
            .filter(|&c| c == '#')
            .count()
            .eq(&0)
            .then(|| lines_to_expand.push(i));
    }

    for g1 in galaxies1.clone() {
        for &g2 in galaxies1.iter() {
            if g1 == g2 {
                break;
            }

            let distance = usize::abs_diff(g1.0, g2.0) + usize::abs_diff(g1.1, g2.1);
            let mut extra = 0;

            for &x in &lines_to_expand {
                if x > usize::min(g1.0, g2.0) && x < usize::max(g1.0, g2.0) {
                    extra += 1;
                }
            }

            for &x in &columns_to_expand {
                if x > usize::min(g1.1, g2.1) && x < usize::max(g1.1, g2.1) {
                    extra += 1;
                }
            }

            result1 += distance + extra;
            result2 += distance + extra * 999_999;
        }
    }

    println!("Result 1: {}", result1); // Result 1: 9648398
    println!("Result 2: {}", result2); // Result 2: 618800410814
}
