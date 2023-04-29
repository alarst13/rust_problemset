use std::io;

// reads input strings in the format of x, y, z, ..., and converts the strings into a vector of integers x, y, z, ....
fn read_in_row(_n: Option<usize>) -> Vec<usize> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let trimmed = line.trim();
    let vec: Vec<usize> = trimmed.split(" ").map(|s| s.parse::<usize>().unwrap()).collect();
    vec
}

fn main() {
    let t = read_in_row(None)[0];

    for _ in 0..t {
        let mut sum: i32 = 0;
        let n = read_in_row(None)[0];
        let mut seq = Vec::new();
        seq.append(&mut read_in_row(Some(n)));
        seq.sort();

        for i in 0..seq.len() {
            for j in 0..seq.len() {
                if j != i && seq[j] % seq[i] == 0 {
                    let quotient = seq[j] / seq[i];
                    for k in 0..seq.len() {
                        if k != j && k != i && seq[k] == quotient * seq[j]{
                            sum += 1;
                        }
                    }
                }
            }
        }
        println!("{}", sum);
    }
}