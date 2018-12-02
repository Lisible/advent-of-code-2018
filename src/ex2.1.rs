use std::fs::File;
use std::io::*;

fn main() {
    let file = File::open("data/ex2/input").expect("Couldn't open file");
    let reader = BufReader::new(file);

    let mut appearing_twice_count = 0;
    let mut appearing_thrice_count = 0;

    for line in reader.lines() {
        let (twice, thrice) = count_appearance(&line.unwrap());

        appearing_twice_count += twice;
        appearing_thrice_count += thrice;
    }

    println!("Twice: {}, Thrice: {}, Checksum: {}", appearing_twice_count,
                                                    appearing_thrice_count,
                                                    appearing_twice_count * appearing_thrice_count);
}

fn count_appearance(line : &str) -> (u32, u32) {
    use std::collections::HashSet;
    let characters : HashSet<char> = line.chars().clone().collect();

    let mut appearances = (0, 0);

    let mut already_twice = false;
    let mut already_thrice = false;
    for character in characters {
        if !already_thrice && line.chars().filter(|c| *c == character).count() == 3 {
            appearances.1 += 1;
            already_thrice = true;
        } else if !already_twice && line.chars().filter(|c| *c == character).count() == 2 {
            appearances.0 += 1;
            already_twice = true;
        }
    }

    appearances
}