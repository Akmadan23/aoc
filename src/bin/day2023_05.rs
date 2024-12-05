use std::str::FromStr;

struct Map {
    dest_start: usize,
    source_start: usize,
    range_len: usize,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Vec<usize> = s
            .split(" ")
            .filter_map(|x| x.parse().ok())
            .collect();

        Ok(Self {
            dest_start: values[0],
            source_start: values[1],
            range_len: values[2],
        })
    }
}

pub fn main() {
    let groups: Vec<String> = aoc::file_to_vec("data/2023/05.txt", "\n\n").unwrap();

    let mut seeds1: Vec<usize> = groups[0]
        .split(' ')
        .filter_map(|x| x.parse().ok())
        .collect();

    let mut seed_ranges: Vec<(usize, usize)> = seeds1
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[1] + chunk[0]))
        .collect();

    let maps: Vec<Vec<Map>> = groups
        .iter()
        .map(|g| g
            .lines()
            .skip(1)
            .filter_map(|x| x.parse().ok())
            .collect())
        .collect();

    for seed in &mut seeds1 {
        for map_vec in &maps {
            for map in map_vec {
                let source_end = map.source_start + map.range_len;
                if *seed >= map.source_start && *seed < source_end {
                    let offset = *seed - map.source_start;
                    *seed = map.dest_start + offset;
                    break;
                }
            }
        }
    }

    let mut seeds2 = vec![];

    for _ in 0..2 { // 2 or 3 runs should be enough
        let ranges_clone = seed_ranges.clone();
        seed_ranges.clear();

        for (seed_start, seed_end) in ranges_clone {
            for map_vec in &maps {
                let mut lowest = None;

                for map in map_vec {
                    let source_start = map.source_start;
                    let source_end = map.source_start + map.range_len;

                    if seed_start >= source_start && seed_end <= source_end {
                        // source_start <= seed_start <= seed_end <= source_end
                        let offset = seed_start - map.source_start;
                        lowest = Some(map.dest_start + offset);
                        break;
                    } else if seed_start >= source_start && seed_start <= source_end {
                        // source_start <= seed_start <= source_end
                        seed_ranges.push((seed_start, source_end));
                        seed_ranges.push((source_end + 1, seed_end));
                    } else if seed_end >= source_start && seed_end <= source_end {
                        // source_start <= seed_end <= source_end
                        seed_ranges.push((seed_start, source_start - 1));
                        seed_ranges.push((source_start, seed_end));
                    } else if seed_start < source_start && seed_end > source_end {
                        // seed_start <= source_start <= source_end <= seed_end
                        seed_ranges.push((seed_start, source_start - 1));
                        seed_ranges.push((source_start, source_end));
                        seed_ranges.push((source_end + 1, seed_end));
                    }
                }

                if let Some(s) = lowest {
                    seeds2.push(s);
                }
            }
        }
    }

    let result1 = seeds1
        .iter()
        .min()
        .unwrap();

    let result2 = seeds2
        .iter()
        .filter(|&&x| x > 2) // for some reason there are a couple of ones and zeros
        .min()
        .unwrap();

    println!("Result 1: {}", result1); // Result 1: 199602917
    println!("Result 2: {}", result2); // Result 2: 2254686
}
