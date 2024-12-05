use std::collections::HashMap;

pub fn main() {
    let mut lines: Vec<String> = aoc::file_to_vec("data/2023/03.txt", "\n").unwrap();
    let mut result1 = 0;
    let mut gears = HashMap::<(usize, usize), Vec<usize>>::new();

    for line_i in 0..lines.len() {
        lines[line_i].push('.');
        let line = &lines[line_i];
        let mut last_char = '.';
        let mut nr_buf = String::new();

        for (char_i, char) in line.char_indices() {
            if char.is_digit(10) {
                nr_buf.push(char);
            } else if !nr_buf.is_empty() {
                let nr = nr_buf.parse::<usize>().unwrap();
                let mut flag1 = false;
                let mut coordinates = None;

                let start = char_i
                    .checked_sub(nr_buf.len() + 1)
                    .unwrap_or(0);

                let end = match char_i {
                    140 => 139,
                    _ => char_i,
                };

                if char != '.' || last_char != '.' {
                    flag1 = true;
                }

                if char == '*' {
                    coordinates = Some((line_i, char_i));
                }

                if last_char == '*' {
                    coordinates = Some((line_i, start))
                }

                for (line_offset, line) in [
                    (0, lines.get(line_i.checked_sub(1).unwrap_or(150))),
                    (2, lines.get(line_i + 1)),
                ] {
                    if let Some(l) = line {
                        for (char_offset, char) in l[start..=end].char_indices() {
                            if !flag1 && "*/+-=$%#@&".contains(char) {
                                flag1 = true;
                            }

                            if char == '*' {
                                coordinates = Some((line_i + line_offset - 1, start + char_offset));
                            }
                        }
                    }
                }

                if flag1 {
                    result1 += nr;
                }

                if let Some((x, y)) = coordinates {
                    if let Some(vec) = gears.get_mut(&(x, y)) {
                        vec.push(nr);
                    } else {
                        gears.insert((x, y), vec![nr]);
                    }
                }

                nr_buf.clear();
                last_char = char;
            } else {
                last_char = char;
            }

        }
    }

    let result2: usize = gears
        .iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(_, v)| v.iter().fold(1, |acc, x| acc * x))
        .sum();

    println!("Result 1: {}", result1); // Result 1: 530849
    println!("Result 2: {}", result2); // Result 2: 84900879
}
