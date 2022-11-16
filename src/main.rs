mod bin;
use bin::*;
use aoc::{ flush, read };

fn main() {
    println!("Advent of Code: 2020-2021");
    println!("-------------------------"); 

    loop {
        let mut input = String::new();
        print!("\nSelect a year between 2020 and 2021: ");
        flush!();
        read!(input);

        if let Ok(year) = input.trim().parse() {
            match year {
                2020..=2021 => loop {
                    let mut input = String::new();
                    print!("\nSelect a day between 1 and 25: ");
                    flush!();
                    read!(input);

                    if let Ok(day) = input.trim().parse() {
                        match year {
                            2020 => match day {
                                // work in progress
                                0 => break,
                                _ => continue
                            },

                            2021 => match day {
                                1 => day2021_01::main(),
                                2 => day2021_02::main(),
                                3 => day2021_03::main(),
                                4 => day2021_04::main(),
                                5 => day2021_05::main(),
                                6 => day2021_06::main(),
                                7 => day2021_07::main(),
                                0 => break,
                                _ => continue
                            },

                            _ => break
                        }
                    }
                },

                0 => break,
                _ => continue
            }
        }
    }
}
