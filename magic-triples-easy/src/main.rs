#[macro_use]
extern crate readln;

// returns a vector of all numbers that are divisible by seq[start]
fn find_divs(start: usize, seq: Vec<usize>) -> Vec<usize> {
    let mut divs = Vec::new();
    for i in start..seq.len() {
        if seq[i] % seq[start] == 0 {
            divs.push(seq[i])
        }
    }
    divs
}

fn main() {
    let t: usize = read!(usize).unwrap();

    for _ in 0..t {
        let mut n: usize = read!(usize).unwrap();
        let mut seq = Vec::new();
        let mut divs = Vec::new();
        for _ in 0..n {
            let a = read!(usize).unwrap();
            seq.push(a);
        }
        seq.sort();
        println!("{:?}", seq);

        // get different lists of divisibls
        // if a list is a subset of another, do not approve it
        for i in 0..n {
            divs.append(&mut (find_divs(i, seq.clone())));
            let mut p = 0;
            println!("{:?}", divs);
            for j in i..n {
                if divs.contains(&seq[j-p]) {
                    seq.remove(j-p);
                    n -= 1;
                    p += 1;
                }
            }
            divs.clear();
        }
    }
}
