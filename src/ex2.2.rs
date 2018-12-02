use std::fs::File;
use std::io::*;

fn main() {
    let file = File::open("data/ex2/input").expect("Couldn't open file");
    let reader = BufReader::new(file);

    let lines : Vec<String> = reader.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();


    let mut str1 = "";
    let mut str2 = "";
    'outer: for i in 0..lines.len()-1 {
        for j in i..lines.len() {
            str1 = &lines.get(i).unwrap();
            str2 = &lines.get(j).unwrap();
            if difference(&str1, &str2) == 1 {
                break 'outer;
            }
        }
    }

    println!("{}, {}", str1, str2);
}

fn difference(str1: &str, str2: &str) -> u32 {
    let mut diff = 0;

    for i in 0..str1.len() {
        if str1.get(i..i+1).unwrap() != str2.get(i..i+1).unwrap() {
            diff += 1;
        }
    }

    println!("{}", diff);

    diff
}