use std::{
    thread,
    collections::VecDeque
};

pub fn main() {
    let mut counts = VecDeque::from(vec![0; 9]);
    let mut day = 0;

    aoc::file_to_vec("data/2021/06.txt", ",")
        .unwrap()
        .into_iter()
        .for_each(|i| counts[i] += 1);

    loop {
        let new = counts.pop_front().unwrap();
        counts[6] += new;
        counts.push_back(new);
        day += 1;

        match day {
            80  => println!("Result 1: {}", counts.iter().sum::<u64>()), // Result 1: 345387
            256 => println!("Result 2: {}", counts.iter().sum::<u64>()), // Result 2: 1574445493136
            257 => break,
            _ => continue
        }
    }
}

// This was the first implementation, for 80 days it worked fine even in a single thread,
// but for 256 days the complexity goes crazy and it takes really long.
// I'll keep it here because anyways this multi-threaded version is pretty cool
fn _simulation() {
    let mut v: Vec<u8> = aoc::file_to_vec("data/2021/06.txt", ",").unwrap();

    for day in 1..=256 {
        let mut thr = vec![];

        let slices: Vec<Vec<u8>> = v
            .chunks(v.len() / 4)
            .map(|i| i.to_vec())
            .collect();

        for mut slice in slices {
            let t = thread::spawn(move || {
                for i in 0..slice.len() {
                    if slice[i] == 0 {
                        slice[i] = 6;
                        slice.push(8);
                    } else {
                        slice[i] -= 1;
                    }
                }

                return slice;
            });

            thr.push(t);
        }

        v.clear();

        for t in thr {
            v.append(&mut t.join().unwrap());
        }

        println!("Day {}: {}", day + 1, v.len());
    }
}
