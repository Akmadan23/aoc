use std::str::FromStr;

struct Card {
    instances: u32,
    numbers: Vec<u32>,
    winning: Vec<u32>,
}

impl Card {
    fn get_points(&self) -> u32 {
        let mut points = 0;

        for n in &self.numbers {
            if self.winning.contains(n) {
                points += 1;
            }
        }

        points
    }
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers: Vec<u32> = s
            .split_once(':')
            .ok_or(())?.1
            .replace("|", "")
            .split(' ')
            .filter_map(|x| x.parse().ok())
            .collect();

        Ok(Self {
            instances: 1,
            numbers: numbers[10..].to_vec(),
            winning: numbers[..10].to_vec(),
        })
    }
}

pub fn main() {
    let mut cards: Vec<Card> = aoc::read_from_file("data/2023/04.txt", "\n").unwrap();

    let result1: u32 = cards
        .iter()
        .filter_map(|c| Some(2_u32.pow(c.get_points().checked_sub(1)?)))
        .sum();

    for i in 0..cards.len() {
        let start = i + 1;
        let end = start + cards[i].get_points() as usize;

        for j in start..end {
            cards[j].instances += cards[i].instances;
        }
    }

    let result2: u32 = cards
        .iter()
        .fold(0, |acc, c| acc + c.instances);

    println!("Result 1: {}", result1); // Result 1: 21213
    println!("Result 2: {}", result2); // Result 2: 8549735
}
