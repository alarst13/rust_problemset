use std::io;

fn read_integer() -> i32 {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let trimmed = line.trim().parse::<i32>().unwrap();
    trimmed
}

fn count_ordinary() -> usize {
    let base: usize = 10;
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let trimmed = line.trim().parse::<usize>().unwrap();
    let msf = trimmed/(base.pow((line.len()-3).try_into().unwrap()));
    let mut next_ordinary: usize = 0;
    for i in 0..line.len()-2 {
        next_ordinary = next_ordinary + msf * (base.pow(i.try_into().unwrap()));
    }
    if next_ordinary <= trimmed {
        return (line.len()-3)*9 + trimmed/(base.pow((line.len()-3).try_into().unwrap()));
    }
    else {
        return (line.len()-3)*9 + trimmed/(base.pow((line.len()-3).try_into().unwrap())) - 1;
    }
}

fn main() {
    let mut count;
    let t = read_integer();
    for _ in 1..t+1 {
        count = count_ordinary();
        println!("{}", count);
    }
}