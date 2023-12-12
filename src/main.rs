mod bin;
use bin::*;
use aoc::{ flush, read };

fn main() {
    println!("Advent of Code: 2020..=2023");
    println!("---------------------------");

    loop {
        let mut input = String::new();
        print!("\nSelect a year between 2020 and 2023 (0 to exit): ");
        flush!();
        read!(input);

        let Ok(year) = input.trim().parse() else {
            continue;
        };

        match year {
            2020..=2023 => loop {
                let mut input = String::new();
                print!("\n[{}] Select a day between 1 and 25 (0 to change year): ", year);
                flush!();
                read!(input);

                let Ok(day) = input.trim().parse() else {
                    continue;
                };

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
                        1 => day2022_01::main(),
                        2 => day2022_02::main(),
                        3 => day2022_03::main(),
                        4 => day2022_04::main(),
                        5 => day2022_05::main(),
                        6 => day2022_06::main(),
                        7 => day2022_07::main(),
                        8 => day2022_08::main(),
                        9..=25 => println!("Work in progress..."),
                        0 => break,
                        _ => continue
                    },

                    2023 => match day {
                        1 => day2023_01::main(),
                        2 => day2023_02::main(),
                        3 => day2023_03::main(),
                        4 => day2023_04::main(),
                        5 => day2023_05::main(),
                        6 => day2023_06::main(),
                        7 => day2023_07::main(),
                        8 => day2023_08::main(),
                        9 => day2023_09::main(),
                        10 => day2023_10::main(),
                        11..=25 => println!("Work in progress..."),
                        0 => break,
                        _ => continue
                    },

                    _ => break
                }
            },

            0 => break,
            _ => continue
        }
    }
}
