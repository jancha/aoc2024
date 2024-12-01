use regex::Regex;
use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").expect("Could not read file?");

    let file = file.replace('\n', "");

    let re = Regex::new(r"(.*?)(do\(\)|don't\(\)|$)").unwrap();

    let mut mul_enabled = true;

    let matches: Vec<i32> = re
        .captures_iter(&file)
        .map(|cap| {
            let chunk = cap.get(1).unwrap().as_str();

            let local_sum = if mul_enabled {
                let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
                let matches: Vec<i32> = re
                    .captures_iter(chunk)
                    .map(|cap| {
                        let left: i32 = cap.get(1).unwrap().as_str().parse().unwrap();
                        let right: i32 = cap.get(2).unwrap().as_str().parse().unwrap();
                        left * right
                    })
                    .collect();
                let local_sum: i32 = matches.iter().sum();
                local_sum
            } else {
                0
            };

            let instruction = cap.get(2).unwrap().as_str();
            match instruction {
                "do()" => {
                    mul_enabled = true;
                }
                "don't()" => {
                    mul_enabled = false;
                }
                _ => {}
            }
            local_sum
        })
        .collect();
    let sum: i32 = matches.iter().sum();

    println!("{}", sum);
}
