fn eval(time: usize, duration: usize) -> u32 {
    let mut count: u32 = 0;
    for i in 2..time {
        if i * (time - i) > duration {
            count += 1;
        }
    }

    return count;
}

pub fn main() {
    let lines: Vec<String> = aoc::file_to_vec("data/2023/06.txt", "\n").unwrap();

    let times_and_durations_pt1 = |index: usize| lines[index]
        .split(' ')
        .filter_map(|x| x.parse().ok())
        .collect();

    let times: Vec<usize> = times_and_durations_pt1(0);
    let durations: Vec<usize> = times_and_durations_pt1(1);

    let result1 = times
        .iter()
        .zip(&durations)
        .map(|(&t, &d)| eval(t, d))
        .fold(1, |acc, x| acc * x);

    let time_and_duration_pt2 = |index: usize| lines[index]
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse()
        .unwrap();

    let time2: usize = time_and_duration_pt2(0);
    let duration2: usize = time_and_duration_pt2(1);
    let result2 = eval(time2, duration2);

    println!("Result 1: {}", result1); // Result 1: 114400
    println!("Result 2: {}", result2); // Result 2: 21039729
}
