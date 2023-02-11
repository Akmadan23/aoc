mod bin;
use bin::*;
use aoc::{ flush, read };

fn main() {
    println!("Advent of Code: 2020..=2022");
    println!("---------------------------");

    loop {
        let mut input = String::new();
        print!("\nSelect a year between 2020 and 2022 (0 to exit): ");
        flush!();
        read!(input);

        if let Ok(year) = input.trim().parse() {
            match year {
                2020..=2022 => loop {
                    let mut input = String::new();
                    print!("\n[{}] Select a day between 1 and 25: ", year);
                    flush!();
                    read!(input);

                    if let Ok(day) = input.trim().parse() {
                        match year {
                            2020 => match day {
                                1 => day2020_01::main(),
                                2 => day2020_02::main(),
                                3 => day2020_03::main(),
                                4 => day2020_04::main(),
                                5..=25 => println!("Work in progress..."),
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
                                8..=25 => println!("Work in progress..."),
                                0 => break,
                                _ => continue
                            },

                            2022 => match day {
                                1..=25 => println!("Work in progress..."),
                                0 => break,
                                _ => continue
                            }

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
