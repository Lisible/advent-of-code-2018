use std::fs::File;
use std::io::*;
use std::str::FromStr;

fn main() {
    let file = File::open("data/ex1/input").unwrap();
    let reader = BufReader::new(file);

    let mut frequency = 0;
    for line in reader.lines() {
        frequency += i32::from_str(&line.unwrap()).unwrap();
    }

    println!("{}", frequency);
}
