use std::{
    str::FromStr,
    ops::Range,
};

const SIZE: usize = 130;

struct Cell {
    starting: bool,
    obstacle: bool,
    looping_obstacle: bool,
    visited: bool,
}

impl FromStr for Cell {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [mut obstacle, mut starting] = [false; 2];

        match s {
            "#" => obstacle = true,
            "^" => starting = true,
            _ => (),
        }

        Ok(Self {
            starting,
            obstacle,
            looping_obstacle: false,
            visited: false,
        })
    }
}

fn find_segment(x: usize, y: usize, data: &Vec<Vec<Cell>>, vert: bool) -> Range<usize> {
    let (mut start, mut end) = (0, SIZE);
    let (x, y, z) = if vert {
        (None, Some(y), x)
    } else {
        (Some(x), None, y)
    };

    for i in (0..z).rev() {
        if data[x.unwrap_or(i)][y.unwrap_or(i)].obstacle {
            start = i;
            break;
        }
    }

    for i in z..SIZE {
        if data[x.unwrap_or(i)][y.unwrap_or(i)].obstacle {
            end = i;
            break;
        }
    }

    start..end
}

pub fn main() {
    let mut data: Vec<Vec<Cell>> = aoc::file_to_matrix("data/2024/06.txt", "\n", "").unwrap();
    let mut result1 = 0;
    let [mut x, mut y, mut old_x, mut old_y] = [0; 4];

    'start: for i in 0..SIZE {
        for j in 0..SIZE {
            if data[i][j].starting {
                x = i;
                y = j;
                break 'start;
            }
        }
    }
    
    // match (dir % 4)
    // 0 -> Up
    // 1 -> Right
    // 2 -> Down
    // 3 -> Left
    let mut dir: usize = 0;
    let mut dir_changed = false;
    let mut next_looping = false;
    let mut looping_ranges: [Vec<(usize, Range<usize>)>; 4] = [vec![], vec![], vec![], vec![]];
    // let mut looping_ranges: [HashSet<(usize, usize)>; 4] = [HashSet::new(), HashSet::new(), HashSet::new(), HashSet::new()];

    while data.get(x).is_some_and(|row| row.get(y).is_some()) {
        let (variable_xy, constant_xy, range) = if dir % 2 == 0 {
            (x, y, find_segment(x, y, &data, true))
        } else {
            (y, x, find_segment(x, y, &data, false))
        };

        let cur = &mut data[x][y];
        // let dir = dir_count % 4;

        if cur.obstacle {
            x = old_x;
            y = old_y;
            dir += 1;
            dir_changed = true;
            next_looping = false;
            continue;
        }

        if next_looping {
            next_looping = false;

            if !cur.starting {
                cur.looping_obstacle = true;
                // println!("{x}, {y}");
            }
        }

        if (cur.starting && !cur.visited) || dir_changed {
            dir_changed = false;
            looping_ranges[dir % 4].push((constant_xy, range));
            // dbg!(&looping_ranges[dir % 4].last().unwrap());
        }

        if !cur.visited {
            cur.visited = true;
            result1 += 1;
        }

        for (constant, range) in &looping_ranges[(dir + 1) % 4] {
            if constant == &variable_xy && range.contains(&constant_xy) {
                next_looping = true;
                break;
            }
        }

        old_x = x;
        old_y = y;

        match dir % 4 {
            0 => x -= 1,
            1 => y += 1,
            2 => x += 1,
            3 => y -= 1,
            _ => (),
        }
    }

    let result2: usize = data
        .iter()
        .map(|row| row
            .iter()
            .filter(|x| x.looping_obstacle)
            .count())
        .sum();

    println!("Result 1: {}", result1); // Result 1: 5329
    println!("Result 2: {}", result2); // Result 2: > 624, !676, !693, 508?
}
