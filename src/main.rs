mod bin;
use bin::*;
use lib::{ flush, read };

// All the puzzles descriptions are here
// https://adventofcode.com/2021

fn main() {
    println!("Advent of Code: 2021");
    println!("--------------------"); 

    loop {
        let mut input = String::new();
        print!("\nSelect day: ");
        flush!();
        read!(input);

        match input.trim() {
            "1" => day1::main(),
            "2" => day2::main(),
            "3" => day3::main(),
            "4" => day4::main(),
            "0" => break,
            _ => continue
        }
    }
}
