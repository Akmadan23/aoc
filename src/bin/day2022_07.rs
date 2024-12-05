use std::{
    str::FromStr,
    collections::HashMap
};

#[derive(Debug)]
enum Command {
    Cd(String),
    Ls(Vec<u32>)
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once("\n") {
            Some(("ls", output)) => Ok(Self::Ls(output
                .split("\n")
                .filter_map(|s| Some(s
                    .split(" ")
                    .next()?
                    .parse()
                    .ok()?))
                .collect())),

            _ => match s.split_once(" ") {
                Some(("cd", arg)) => Ok(Self::Cd(arg.to_string())),
                _ => Err(())
            }
        }
    }
}

pub fn main() {
    let commands: Vec<Command> = aoc::file_to_vec("data/2022/07.txt", "$ ").unwrap();
    let mut sizes = HashMap::new();
    let mut path = Vec::new();

    for c in &commands {
        match c {
            Command::Cd(new_path) => match new_path.as_str() {
                ".." => _ = path.pop().unwrap(),
                dir => {
                    if let Some(partial) = path.last() {
                        let x = format!("{partial}{dir}/");
                        path.push(x.clone());
                        sizes.insert(x, 0);
                    } else {
                        path.push(dir.to_string());
                        sizes.insert(dir.to_string(), 0);
                    }
                }
            },

            Command::Ls(files) => for size in files {
                for p in &path {
                    if let Some(dir) = sizes.get_mut(p) {
                        *dir += size;
                    }
                }
            }
        }
    }

    let needed_storage = 30_000_000 - (70_000_000 - sizes["/"]);

    let result1: u32 = sizes
        .values()
        .filter(|&&v| v <= 100_000)
        .sum();

    let result2 = sizes
        .values()
        .filter(|&&v| v >= needed_storage)
        .min()
        .unwrap();

    println!("Result 1: {}", result1); // Result 1: 1555642
    println!("Result 2: {}", result2); // Result 2: 5974547
}
