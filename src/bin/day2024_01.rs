pub fn main() {
    let lines: Vec<String> = aoc::file_to_vec("data/2024/01.txt", "\n").unwrap();
    let (mut result1, mut result2) = (0, 0);

    let (mut list1, mut list2): (Vec<u32>, Vec<u32>) = lines
        .iter()
        .filter_map(|l| l.split_once("   "))
        .filter_map(|t| Some((
            t.0.parse::<u32>().ok()?,
            t.1.parse::<u32>().ok()?
        )))
        .unzip();

    list1.sort();
    list2.sort();

    list1
        .iter()
        .zip(&list2)
        .for_each(|(x, y)| result1 += x.abs_diff(*y));

    list1
        .iter()
        .for_each(|&x| result2 += x as usize * list2
            .iter()
            .filter(|&&y| y == x)
            .count());

    println!("Result 1: {}", result1); // Result 1: 1189304
    println!("Result 2: {}", result2); // Result 2: 24349736
}
