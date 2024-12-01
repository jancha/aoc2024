use std::fs;

fn main() {
    // read file, sort while reading maybe?
    // generate two vectors
    // get sums

    let mut left_list = vec![];
    let mut right_list = vec![];

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
        right_list.push(value_right);
    }

    left_list.sort();
    right_list.sort();

    let mut sum = 0;

    for (row, left) in left_list.iter().enumerate() {
        let right = right_list.get(row).unwrap();
        sum += (left - right).abs();
    }

    println!("{}", sum);
}
