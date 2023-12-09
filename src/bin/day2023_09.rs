use std::{
    str::FromStr,
    ops::Deref,
};

struct Sequence(Vec<isize>);

impl Deref for Sequence {
    type Target = Vec<isize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Sequence {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s
            .split(' ')
            .filter_map(|c| c.parse().ok())
            .collect()))
    }
}

pub fn main() {
    let sequences: Vec<Sequence> = aoc::read_from_file("data/2023/09.txt", "\n").unwrap();
    let [mut result1, mut result2]: [isize; 2] = [0; 2];

    let get_diffs = |x: &Vec<isize>| x
        .windows(2)
        .map(|w| w[1] - w[0])
        .collect::<Vec<isize>>();

    for seq in &sequences {
        let mut diffs = get_diffs(&seq);
        let mut firsts = vec![seq.first().copied()];
        let mut lasts = vec![seq.last().copied()];

        loop {
            let all_zeros = diffs
                .iter()
                .filter(|&&x| x != 0)
                .count()
                .eq(&0);

            if all_zeros {
                result1 += lasts
                    .iter()
                    .flatten()
                    .sum::<isize>();

                result2 += firsts
                    .iter()
                    .flatten()
                    .enumerate()
                    .fold(0, |acc, (index, elem)| match index % 2 {
                        0 => acc + elem,
                        1 => acc - elem,
                        _ => panic!()
                    });
                break;
            } else {
                firsts.push(diffs.first().copied());
                lasts.push(diffs.last().copied());
                diffs = get_diffs(&diffs);
            }
        }
    }

    println!("Result 1: {}", result1); // Result 1: 1647269739
    println!("Result 2: {}", result2); // Result 2: 84900879
}
