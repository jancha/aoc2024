use regex::Regex;
use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").expect("Could not read file?");

    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let matches: Vec<i32> = re
        .captures_iter(&file)
        .map(|cap| {
            let left: i32 = cap.get(1).unwrap().as_str().parse().unwrap();
            let right: i32 = cap.get(2).unwrap().as_str().parse().unwrap();
            left * right
        })
        .collect();
    let sum: i32 = matches.iter().sum();

    println!("{}", sum);
}
