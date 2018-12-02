use std::fs::File;
use std::io::*;
use std::collections::HashSet;
use std::str::FromStr;

fn main() {
    let file = File::open("data/ex1/input").unwrap();
    let reader = BufReader::new(file);

    let lines : Vec<String> = reader.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    let mut frequency = 0;
    let mut already_reached_frequencies = HashSet::new();
    already_reached_frequencies.insert(frequency);

    let mut it = lines.iter().cycle();
    loop {
        frequency += i32::from_str(it.next().unwrap()).unwrap();
        if already_reached_frequencies.contains(&frequency) {
            break;
        }
        else {
            already_reached_frequencies.insert(frequency);
        }
    }

    println!("{}", frequency);
}
