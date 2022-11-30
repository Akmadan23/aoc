use std::{
    str::FromStr,
    convert::Infallible
};

#[derive(Default)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
    valid: bool
}

impl FromStr for Passport {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut p = Passport::default();
        let data: Vec<Vec<String>> = s
            .replace("\n", " ")
            .split(" ")
            .map(|i| i
                .split(":")
                .map(|i| i.to_string())
                .collect())
            .collect();

        for item in &data {
            let value = Some(item[1].clone());

            match item[0].as_str() {
                "byr" => p.byr = value,
                "iyr" => p.iyr = value,
                "eyr" => p.eyr = value,
                "hgt" => p.hgt = value,
                "hcl" => p.hcl = value,
                "ecl" => p.ecl = value,
                "pid" => p.pid = value,
                "cid" => p.cid = value,
                _ => ()
            }
        }

        match data.len() {
            8 => p.valid = true,
            7 => p.valid = p.cid == None,
            _ => ()
        }

        Ok(p)
    }
}

pub fn main() {
    let passports: Vec<Passport> = aoc::read_from_file("data/2020/04.txt", "\n\n").unwrap();
    let colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    let [mut result1, mut result2] = [0; 2];

    'p: for p in passports {
        if p.valid {
            result1 += 1;

            if let (Some(byr), Some(iyr), Some(eyr)) = (p.byr, p.iyr, p.eyr) {
                let years = [
                    (byr, 1920..=2002),
                    (iyr, 2010..=2020),
                    (eyr, 2020..=2030),
                ];

                for (key, range) in years {
                    match key.parse() {
                        Ok(x) => if !range.contains(&x) {
                            continue 'p
                        },
                        Err(_) => continue 'p
                    }
                }
            }

            if let Some(hgt) = p.hgt {
                let (height, unit) = hgt.split_at(hgt.len() - 2);

                let range = match unit {
                    "cm" => 150..=193,
                    "in" => 59..=76,
                    _ => continue 'p
                };

                match height.parse() {
                    Ok(x) => if !range.contains(&x) {
                        continue 'p
                    },
                    Err(_) => continue 'p
                }
            }

            if let Some(hcl) = p.hcl {
                if hcl.len() == 7 {
                    for (char, index) in hcl.chars().zip(0..) {
                        if index == 0 {
                            if char != '#' {
                                continue 'p
                            }
                        } else if char.to_digit(16) == None {
                            continue 'p
                        }
                    }
                } else {
                    continue 'p
                }
            }

            if let Some(pid) = p.pid {
                if pid.len() == 9 {
                    for char in pid.chars() {
                        if char.to_digit(10) == None {
                            continue 'p
                        }
                    }
                } else {
                    continue 'p
                }
            }

            if let Some(ecl) = p.ecl {
                if !colors.contains(&ecl.as_str()) {
                    continue 'p
                }
            }

            result2 += 1;
        }
    }

    println!("Result 1: {}", result1); // Result 1: 237
    println!("Result 2: {}", result2); // Result 2: 172
}
