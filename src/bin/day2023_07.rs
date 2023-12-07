use std::{
    cmp::Ordering,
    str::FromStr,
    num::ParseIntError,
    collections::HashMap,
};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
enum CardType {
    Joker,
    Nr(u32),
    Jack,
    Queen,
    King,
    Ace,
}

impl From<char> for CardType {
    fn from(value: char) -> Self {
        match value.to_digit(10) {
            Some(x) => Self::Nr(x),
            None => match value {
                'T' => Self::Nr(10),
                'J' => Self::Jack,
                'Q' => Self::Queen,
                'K' => Self::King,
                'A' => Self::Ace,
                _ => panic!("Unexpected card"),
            }
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    Pair,
    DoublePair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

struct Hand {
    type1: HandType,
    type2: HandType,
    cards1: Vec<CardType>,
    cards2: Vec<CardType>,
    bid: usize,
}

impl FromStr for Hand {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_once(' ').unwrap();
        let mut map = HashMap::<char, u8>::new();

        for c in cards.chars() {
            if let Some(x) = map.get_mut(&c) {
                *x += 1;
            } else {
                map.insert(c, 1);
            }
        }

        let eval_type = |count: &[u8]| match count[..] {
            [.., 5] => HandType::FiveKind,
            [.., 4] => HandType::FourKind,
            [.., 2, 3] => HandType::FullHouse,
            [.., 3] => HandType::ThreeKind,
            [.., 2, 2] => HandType::DoublePair,
            [.., 2] => HandType::Pair,
            _ => HandType::HighCard,
        };

        let mut counters: Vec<u8> = map
            .values()
            .map(|x| x.clone())
            .collect();

        counters.sort();
        let type1 = eval_type(&counters);

        if let Some(jokers) = map.get(&'J') {
            let len = counters.len();

            if len > 1 {
                for c in &mut counters {
                    if c == jokers {
                        *c = 0;
                        break;
                    }
                }

                if counters[len - 1] != 0 {
                    counters[len - 1] += jokers;
                } else {
                    counters[len - 2] += jokers;
                }
            }
        }

        counters.sort();
        let type2 = eval_type(&counters);

        let cards1: Vec<_> = cards
            .chars()
            .map(|card| card.into())
            .collect();

        let cards2: Vec<_> = cards1
            .iter()
            .map(|card| match card {
                CardType::Jack => CardType::Joker,
                _ => card.clone(),
            })
            .collect();

        Ok(Self {
            type1,
            type2,
            cards1,
            cards2,
            bid: bid.parse()?,
        })
    }
}

macro_rules! sort_method {
    ($type: tt, $cards: tt) => {
        |h1, h2| match h1.$type.cmp(&h2.$type) {
            Ordering::Equal => {
                for (c1, c2) in h1.$cards.iter().zip(&h2.$cards) {
                    match c1.cmp(&c2) {
                        Ordering::Equal => continue,
                        y => return y,
                    }
                }

                Ordering::Equal
            },
            x => x
        }
    };
}

pub fn main() {
    let mut hands: Vec<Hand> = aoc::read_from_file("data/2023/07.txt", "\n").unwrap();
    
    let get_result = |vec: &Vec<Hand>| vec
        .iter()
        .enumerate()
        .fold(0, |acc, (index, hand)| acc + (index + 1) * hand.bid);

    hands.sort_by(sort_method!(type1, cards1));
    let result1: usize = get_result(&hands);

    hands.sort_by(sort_method!(type2, cards2));
    let result2: usize = get_result(&hands);
    
    println!("Result 1: {}", result1); // Result 1: 248569531
    println!("Result 2: {}", result2); // Result 2: 250382098
}
