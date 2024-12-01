use std::collections::HashMap;
use std::fs;

fn main() {
    // read file, sort while reading maybe?
    // generate two vectors
    // get sums

    let mut left_list = vec![];

    let mut right_list_summarized: HashMap<i32, i32> = HashMap::new();

    let file = fs::read_to_string("input.txt").expect("Could not read file?");

    let lines = file.split("\n");

    for i in lines {
        if i.is_empty() {
            continue;
        }
        let line: Vec<&str> = i.trim().split("   ").collect();

        let value_left: i32 = line.first().unwrap().parse().unwrap();
        let value_right: i32 = line.last().unwrap().parse().unwrap();

        left_list.push(value_left);

        let count = if right_list_summarized.contains_key(&value_right) {
            *right_list_summarized.get(&value_right).unwrap()
        } else {
            0
        };

        if count != 0 {
            right_list_summarized.remove(&value_right);
        }
        right_list_summarized.insert(value_right, count + 1);
    }

    let mut sum: i64 = 0;

    for left in left_list {
        if let Some(encounters) = right_list_summarized.get(&left) {
            sum += (left as i64) * (*encounters as i64);
        }
    }

    println!("{}", sum);
}
