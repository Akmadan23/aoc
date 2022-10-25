pub fn main() {
    let v: Vec<i32> = lib::read_from_file("data/1.txt", "\n").unwrap();
    let [mut c1, mut c2] = [0; 2];

    for i in 1..v.len() {
        if v[i] > v[i-1] {
            c1 += 1;
        }

        if i > 2 && v[i] > v[i-3] {
            c2 += 1;
        }
    }

    println!("Result 1: {c1}"); // Result 1: 1521
    println!("Result 2: {c2}"); // Result 2: 1543
}
