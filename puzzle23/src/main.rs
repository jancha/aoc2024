use std::collections::{HashMap, HashSet};
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

    let mut result = 0;

    let mut visited = HashMap::new();

    for (index, c) in map_binary.iter().enumerate() {
        if !visited.contains_key(&index) {
            result += find_price(c, &index, map_binary, &map_width, &map_height, &mut visited);
        }
    }

    result
}

fn find_price(
    c: &u8,
    index: &usize,
    map_binary: &[u8],
    map_width: &usize,
    map_height: &usize,
    visited: &mut HashMap<usize, usize>,
) -> usize {
    let (x, y) = index_to_xy(index, map_width);

    let mut area: HashMap<usize, usize> = HashMap::new();

    explore_area(c, &x, &y, map_binary, map_width, map_height, &mut area);

    let perimeter: usize = area.values().sum();
    let area_size = area.len();

    area.into_iter().for_each(|(i, _j)| {
        visited.insert(i, 1);
    });

    area_size * perimeter
}

fn explore_area(
    c: &u8,
    x: &usize,
    y: &usize,
    map_binary: &[u8],
    map_width: &usize,
    map_height: &usize,
    area: &mut HashMap<usize, usize>,
) -> usize {
    let index = xy_to_index(x, y, map_width);

    if area.get(&index).is_some() {
        //already on our map
        return 1; // end of path
    }

    let c_new = map_binary[index];

    if c_new != *c {
        return 0; // not continous;
    }

    area.insert(index, 4);

    let mut neighbours_found = 0;
    if *x > 0 {
        neighbours_found += explore_area(c, &(*x - 1), y, map_binary, map_width, map_height, area);
    }
    if *x < map_width - 1 {
        neighbours_found += explore_area(c, &(*x + 1), y, map_binary, map_width, map_height, area);
    }
    if *y > 0 {
        neighbours_found += explore_area(c, x, &(*y - 1), map_binary, map_width, map_height, area);
    }
    if *y < map_height - 1 {
        neighbours_found += explore_area(c, x, &(*y + 1), map_binary, map_width, map_height, area);
    }
    let r = area.get_mut(&index).unwrap();

    *r -= neighbours_found;

    1
}

fn index_to_xy(index: &usize, map_width: &usize) -> (usize, usize) {
    (index % map_width, index / map_width)
}
fn xy_to_index(x: &usize, y: &usize, map_width: &usize) -> usize {
    y * map_width + x
}

#[test]
fn test_1() {
    assert_eq!(analyze("test.txt"), 1930);
    assert_eq!(analyze("input.txt"), 0);
}
