use std::collections::HashMap;
use std::fs;

fn main() {
    println!("{}", analyze("input.txt", 100));
}

const WALL: u8 = 35;

fn analyze(file: &str, min_cheat_gain: isize) -> usize {
    let file = fs::read_to_string(file).expect("Could not read file?");
    let data: Vec<&str> = file.trim().split("\n").map(|x| x.trim()).collect();
    let mut start_index = 0;
    let mut end_index = 0;
    let map_width = data[0].len();
    let map_height = data.len();

    let mut map_binary: [isize; 141 * 141] = [-1; 141 * 141];

    for (line, i) in data.iter().enumerate() {
        for (pos, char) in i.char_indices() {
            let index = line * map_width + pos;

            if char == WALL as char {
                map_binary[index] = -1;
            } else {
                map_binary[index] = 0;
            }

            if char == 'S' {
                start_index = index;
            } else if char == 'E' {
                end_index = index;
            }
        }
    }

    drive(
        &mut map_binary,
        &map_width,
        &map_height,
        &start_index,
        &end_index,
        min_cheat_gain,
    )
}

fn drive(
    map_binary: &mut [isize; 141 * 141],
    map_width: &usize,
    map_height: &usize,
    start: &usize,
    end: &usize,
    min_cheat_gain: isize,
) -> usize {
    let mut cheats: HashMap<usize, isize> = HashMap::new();
    let mut index = *start;

    let mut x = start % map_width;
    let mut y = start / map_width;

    let moves: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let cheat_moves: [(isize, isize); 4] = [(-2, 0), (2, 0), (0, -2), (0, 2)];

    let mut cost = 0;
    loop {
        for (dx, dy) in moves {
            let ndx = x as isize + dx;
            if ndx < 0 || ndx as usize >= *map_width {
                continue;
            }
            let ndy = y as isize + dy;
            if ndy < 0 || ndy as usize >= *map_height {
                continue;
            }
            let next_index = xy_to_index(&(ndx as usize), &(ndy as usize), map_width);
            if map_binary[next_index] != 0 {
                continue;
            }
            cost += 1;
            index = next_index;
            map_binary[index] = cost;
            x = ndx as usize;
            y = ndy as usize;
            break;
        }
        // check for cheats
        for (dx, dy) in cheat_moves {
            let ndx = x as isize + dx;
            if ndx < 0 || ndx as usize >= *map_width {
                continue;
            }
            let ndy = y as isize + dy;
            if ndy < 0 || ndy as usize >= *map_height {
                continue;
            }
            let prev_index = xy_to_index(&(ndx as usize), &(ndy as usize), map_width);

            if map_binary[prev_index] < cost && (map_binary[prev_index] > 0 || prev_index == *start)
            {
                let delta = cost - map_binary[prev_index] - 2;

                if delta < min_cheat_gain || delta <= 0 {
                    continue;
                }
                let hash_index = index + (prev_index << 16);

                cheats.entry(hash_index).or_insert(delta);
            }
        }

        if index == *end {
            break;
        }
    }
    cheats.len()
}
fn xy_to_index(x: &usize, y: &usize, map_width: &usize) -> usize {
    y * map_width + x
}

#[test]
fn test_1() {
    assert_eq!(analyze("test.txt", 0), 44);
}
