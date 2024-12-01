use std::fs;

fn main() {
    // read file, sort while reading maybe?
    // generate two vectors
    // get sums

    let file = fs::read_to_string("input.txt").expect("Could not read file?");

    let lines = file.split("\n");

    let mut count = 0;
    for i in lines {
        if i.is_empty() {
            continue;
        }
        let levels: Vec<&str> = i.trim().split(" ").collect();
        let level_count = levels.len();

        for j in 0..level_count + 1 {
            let mut prev_level = None;
            let mut prev_direction = None;
            let mut level_valid = true;
            let mut levels = levels.clone();
            if j > 0 {
                levels.remove(j - 1);
            }
            for i in levels {
                let i: i64 = i.parse().unwrap();
                if prev_level.is_some() {
                    let delta: i64 = i - prev_level.unwrap();
                    let delta_abs = delta.abs();

                    if !(1..=3).contains(&delta_abs) {
                        level_valid = false;
                        break; // invalid change
                    } else {
                        prev_level = Some(i);
                    }

                    let direction = delta < 0;
                    if let Some(prev_direction_value) = prev_direction {
                        if prev_direction_value != direction {
                            level_valid = false;
                        }
                    } else {
                        prev_direction = Some(direction);
                    }
                } else {
                    prev_level = Some(i);
                }
            }
            if level_valid {
                count += 1;
                break;
            }
        }
    }
    println!("{}", count);
}
