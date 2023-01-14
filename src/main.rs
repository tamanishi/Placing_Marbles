use std::io;

fn main() {
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line);
    let values = line
        .trim()
        .chars()
        .collect::<Vec<char>>()
        .iter()
        .map(|e| e.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    println!("{}", values.iter().sum::<u32>());
}
