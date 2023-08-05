use std::{
    ops::Deref,
    str::FromStr,
    convert::Infallible,
};
    
const SIZE: usize = 12;

#[derive(Clone)]
struct Line(Vec<u16>);

#[derive(Default)]
struct Data {
    int: i32,
    str: String,
    lines: Vec<Line>
}

impl Deref for Line {
    type Target = Vec<u16>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Line {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s
            .split("")
            .filter_map(|i| i.parse().ok())
            .collect()))
    }
}

fn get_str(mut lines: Vec<Line>, val1: u16, val2: u16) -> String {
    for i in 0..SIZE {
        let mut cont = 0;
        let len = lines.len() as u16;

        if len == 1 {
            break;
        }

        for l in lines.iter() {
            cont += l[i];
        }

        lines = lines
            .into_iter()
            .filter(|l| if cont >= len - cont {
                l[i] == val1
            } else {
                l[i] == val2
            })
            .collect();
    }

    return lines[0]
        .iter()
        .map(|i| i.to_string())
        .collect();
}

pub fn main() {
    let lines: Vec<Line> = aoc::read_from_file("data/2021/03.txt", "\n").unwrap();

    let [
        mut gamma,
        mut epsilon,
        mut oxygen,
        mut co2,
    ]: [Data; 4];

    gamma = Data::default();
    epsilon = Data::default();
    oxygen = Data::default();
    co2 = Data::default();

    let mut count = [0; SIZE];

    for line in &lines {
        if line.len() > 0 {
            for i in 0..SIZE {
                count[i] += line[i]
            }
        }
    }

    oxygen.lines = lines.clone();
    co2.lines = lines.clone();

    for i in 0..SIZE {
        if count[i] > 500 {
            gamma.str.push('1');
            epsilon.str.push('0');
        } else {
            gamma.str.push('0');
            epsilon.str.push('1');
        }
    }

    oxygen.str = get_str(oxygen.lines, 1, 0);
    co2.str = get_str(co2.lines, 0, 1);

    gamma.int = i32::from_str_radix(&gamma.str, 2).unwrap();
    epsilon.int = i32::from_str_radix(&epsilon.str, 2).unwrap();
    oxygen.int = i32::from_str_radix(&oxygen.str, 2).unwrap();
    co2.int = i32::from_str_radix(&co2.str, 2).unwrap();

    println!("Result 1: {}", gamma.int * epsilon.int);  // Result 1: 2724524
    println!("Result 2: {}", oxygen.int * co2.int);     // Result 2: 2775870
}
