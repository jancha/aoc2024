use std::fs;

fn main() {
    println!("{}", analyze("input.txt"));
}

fn analyze(file: &str) -> u64 {
    let file = fs::read_to_string(file).expect("Could not read file?");
    let map: Vec<&str> = file.trim().split("\n").collect();

    map.iter()
        .map(|x| {
            let x: Vec<&str> = x.trim().split(':').collect();

            let target_val: u64 = x.first().unwrap().parse().unwrap();

            let elements: Vec<u64> = x
                .last()
                .unwrap()
                .trim()
                .split(' ')
                .collect::<Vec<&str>>()
                .into_iter()
                .map(|x| {
                    let n: u64 = x.parse().unwrap();
                    n
                })
                .collect();

            if try_solve(target_val, &elements, *(elements.first().unwrap()), 1).is_some() {
                target_val
            } else {
                0
            }
        })
        .sum()
}

fn try_solve(target_val: u64, elements: &Vec<u64>, prev_val: u64, index: usize) -> Option<u64> {
    let current_elem = elements[index];

    let new_add_val = prev_val + current_elem;

    let new_mul_val = prev_val * current_elem;

    if index < elements.len() - 1 {
        match try_solve(target_val, elements, new_add_val, index + 1) {
            None => try_solve(target_val, elements, new_mul_val, index + 1),
            Some(val) => Some(val),
        }
    } else if target_val == new_mul_val || target_val == new_add_val {
        Some(target_val)
    } else {
        None
    }
}
#[test]
fn test_1() {
    assert_eq!(analyze("test.txt"), 3749);
    assert_eq!(analyze("input.txt"), 465126289353);
}
