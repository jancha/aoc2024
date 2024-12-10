use std::collections::HashSet;
use std::fs;

fn main() {
    println!("{}", analyze("input.txt"));
}

const CHAR_0: u8 = 48;

fn analyze(file: &str) -> usize {
    let file = fs::read_to_string(file).expect("Could not read file?");
    let map: Vec<&str> = file.trim().split("\n").collect();

    let map_width = map.first().unwrap().len();
    let map_height = map.len();

    let map = file.replace("\n", "");
    let map_binary = map.as_bytes();

    let mut trail_heads: Vec<usize> = vec![];

    map_binary.iter().enumerate().for_each(|(index, height)| {
        let height: u8 = *height - CHAR_0;
        if height == 0 {
            trail_heads.push(index);
        }
    });

    trail_heads
        .into_iter()
        .map(|index| find_trails(&index, map_binary, &map_width, &map_height))
        .sum()
}

fn find_trails(index: &usize, map_binary: &[u8], map_width: &usize, map_height: &usize) -> usize {
    let (x, y) = index_to_xy(index, map_width);

    let mut peaks_reached: HashSet<usize> = HashSet::new();

    let trails_found = explore_trails(
        None,
        &x,
        &y,
        map_binary,
        map_width,
        map_height,
        &mut peaks_reached,
    );

    trails_found
}

fn explore_trails(
    prev_height: Option<&u8>,
    x: &usize,
    y: &usize,
    map_binary: &[u8],
    map_width: &usize,
    map_height: &usize,
    peaks_reached: &mut HashSet<usize>,
) -> usize {
    let index = xy_to_index(x, y, map_width);
    let height = map_binary[index] - CHAR_0;

    if let Some(prev_height) = prev_height {
        let height_diff = height as i8 - *prev_height as i8;

        if height_diff != 1 {
            return 0;
        }

        if height == 9 {
            if !peaks_reached.iter().any(|x| *x == index) {
                peaks_reached.insert(index);
                return 1;
            } else {
                return 0;
            }
        }
    }

    let mut trails_found = 0;
    if *x > 0 {
        trails_found += explore_trails(
            Some(&height),
            &(*x - 1),
            y,
            map_binary,
            map_width,
            map_height,
            peaks_reached,
        );
    }
    if *x < map_width - 1 {
        trails_found += explore_trails(
            Some(&height),
            &(*x + 1),
            y,
            map_binary,
            map_width,
            map_height,
            peaks_reached,
        );
    }
    if *y > 0 {
        trails_found += explore_trails(
            Some(&height),
            x,
            &(*y - 1),
            map_binary,
            map_width,
            map_height,
            peaks_reached,
        );
    }
    if *y < map_height - 1 {
        trails_found += explore_trails(
            Some(&height),
            x,
            &(*y + 1),
            map_binary,
            map_width,
            map_height,
            peaks_reached,
        );
    }
    trails_found
}

fn index_to_xy(index: &usize, map_width: &usize) -> (usize, usize) {
    (index % map_width, index / map_width)
}
fn xy_to_index(x: &usize, y: &usize, map_width: &usize) -> usize {
    y * map_width + x
}

#[test]
fn test_1() {
    assert_eq!(analyze("test.txt"), 36);
    assert_eq!(analyze("input.txt"), 501);
}
