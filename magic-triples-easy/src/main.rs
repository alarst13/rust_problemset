use std::io;
fn main() {
    let mut line = String::new();

    io::stdin()
        .read_line(&mut line)
        .unwrap();
    let num_test_cases = line.trim();

    let num_test_cases: u32 = num_test_cases.parse().unwrap();
}