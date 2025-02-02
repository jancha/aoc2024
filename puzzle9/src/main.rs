use std::collections::{hash_map::Entry, HashMap, HashSet};
use std::fs;

#[derive(PartialEq)]
enum ParseMode {
    Rules,
    Sequence,
}

fn main() {
    println!("{}", verify("input.txt"));
}

fn verify(file: &str) -> usize {
    let file = fs::read_to_string(file).expect("Could not read file?");
    let lines: Vec<&str> = file.trim().split("\n").collect();

    let mut must_be_before: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut sequences: Vec<Vec<usize>> = vec![];
    let mut parse_mode = ParseMode::Rules;
    let _tmp: Vec<usize> = lines
        .into_iter()
        .map(|line| {
            if line.is_empty() {
                parse_mode = ParseMode::Sequence;
                return 0;
            }
            if parse_mode == ParseMode::Rules {
                let values: Vec<usize> = line
                    .split("|")
                    .map(|s| {
                        let s: usize = s.parse().unwrap();
                        s
                    })
                    .collect();
                let first = *values.first().unwrap();
                let last = *values.last().unwrap();

                match must_be_before.entry(first) {
                    Entry::Occupied(entry) => {
                        let e = entry.into_mut();
                        e.insert(last);
                    }
                    Entry::Vacant(entry) => {
                        let mut h = HashSet::new();
                        h.insert(last);
                        entry.insert(h);
                    }
                }
            }
            if parse_mode == ParseMode::Sequence {
                let values: Vec<usize> = line
                    .split(",")
                    .map(|s| {
                        let s: usize = s.parse().unwrap();
                        s
                    })
                    .collect();

                let value = validate_sequence(&must_be_before, &values);
                sequences.push(values);
                return value;
            }
            0
        })
        .collect();

    _tmp.iter().sum()
}

fn validate_sequence(must_be_before: &HashMap<usize, HashSet<usize>>, sequence: &[usize]) -> usize {
    let len = sequence.len();
    let mut mid = 0;
    let mid_index = (len as f64 / 2.).round() as usize - 1;
    for i in 1..len {
        let val = sequence.get(i).unwrap();
        if i == mid_index {
            mid = *val;
        }

        for j in i - 1..i {
            let rules = must_be_before.get(val);

            if rules.is_none() {
                continue;
            }

            let rules = rules.unwrap();

            let prev = sequence.get(j).unwrap();
            if rules.get(prev).is_some() {
                // invalid char
                return 0;
            }
        }
    }

    mid
}

#[test]

fn test_1() {
    assert_eq!(verify("test1.txt"), 143);
    assert_eq!(verify("input.txt"), 5374);
}
