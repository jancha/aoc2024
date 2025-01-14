use std::collections::HashMap;
use std::fs;
fn main() {
    println!("{}", analyze("input.txt", 71, 71, 1024));
}

const WALL: u8 = 35;
const EMPTY: u8 = 46;

#[derive(Debug, PartialEq)]
enum Direction {
    East,
    West,
    North,
    South,
}

#[derive(Debug, PartialEq)]
struct Tile {
    start: bool,
    finish: bool,
    cost: usize,
    left: bool,
    right: bool,
    up: bool,
    down: bool,
    x: usize,
    y: usize,
}

impl Tile {
    fn new(x: usize, y: usize, start: bool, finish: bool) -> Tile {
        Tile {
            start,
            finish,
            cost: 0,
            left: false,
            right: false,
            up: false,
            down: false,
            x,
            y,
        }
    }

    fn link_or_create(&mut self, x: usize, y: usize) {
        if x < self.x {
            self.left = true;
        } else if x > self.x {
            self.right = true;
        } else if y < self.y {
            self.up = true;
        } else if y > self.y {
            self.down = true;
        }
    }
}

fn analyze(file: &str, map_width: usize, map_height: usize, read_len: usize) -> String {
    let file = fs::read_to_string(file).expect("Could not read file?");
    let data: Vec<&str> = file.trim().split("\n").map(|x| x.trim()).collect();
    let start_index = 0;
    let end_index = map_width * map_height - 1;

    /*solve_linear(
        &data,
        read_len,
        start_index,
        end_index,
        map_width,
        map_height,
    )*/

    solve_quicksort(
        &data,
        read_len,
        start_index,
        end_index,
        map_width,
        map_height,
    )
}

fn build_map(
    data: &[&str],
    map_width: usize,
    from: usize,
    to: usize,
    tiles: &mut HashMap<usize, Tile>,
    map: &mut [u8],
) {
    for i in &data[from..to] {
        if i.is_empty() {
            continue;
        }

        let mut line = i.split(",");

        let x: usize = line.next().unwrap().parse().unwrap();
        let y: usize = line.next().unwrap().parse().unwrap();

        map[y * map_width + x] = WALL;
    }

    build_graph(map, &map_width, tiles);
}

fn solve_quicksort(
    data: &Vec<&str>,
    read_len: usize,
    start_index: usize,
    end_index: usize,
    map_width: usize,
    map_height: usize,
) -> String {
    let start = 0;
    let data_len = data.len();
    let mut last_found = read_len;
    let mut split = (data_len - read_len) / 2;
    let mut end = read_len + split;
    loop {
        let mut min = 0;
        let mut min_path: Vec<usize> = vec![];
        let mut tiles: HashMap<usize, Tile> = HashMap::new();
        let mut map_binary: Vec<u8> = vec![EMPTY; map_width * map_height];

        println!("Start: {start}, end: {end}");
        build_map(data, map_width, start, end, &mut tiles, &mut map_binary);

        walk_graph(
            &mut min,
            &mut min_path,
            [].to_vec(),
            start_index,
            end_index,
            &mut tiles,
            &map_width,
        );

        if min != 0 {
            last_found = end;
            split = (data_len - last_found) / 2;
            end += split;
            println!("Found solution; {}", last_found - 1);
        } else {
            split = (end - last_found) / 2;
            end -= split;
            println!("No solution so far");
        }

        if split == 0 {
            print_map(&map_binary, map_width);
            println!("Split is zero, braking");
            println!(
                "Last found: {}, Next bit: {:?}",
                last_found - 1,
                data.get(last_found).unwrap()
            );
            return data.get(last_found).unwrap().to_string();
        }
    }
}

fn solve_linear(
    data: &Vec<&str>,
    read_len: usize,
    start_index: usize,
    end_index: usize,
    map_width: usize,
    map_height: usize,
) -> String {
    let mut min = 0;
    let mut min_path: Vec<usize> = vec![];
    let mut map_binary: Vec<u8> = vec![EMPTY; map_width * map_height];
    let mut tiles: HashMap<usize, Tile> = HashMap::new();

    build_map(data, map_width, 0, read_len, &mut tiles, &mut map_binary);

    walk_graph(
        &mut min,
        &mut min_path,
        [].to_vec(),
        start_index,
        end_index,
        &mut tiles,
        &map_width,
    );

    println!("Min: {min}, Min path: {:?}", min_path);

    let mut prev = String::new();

    for bit in 0..data.len() {
        let start = 0;
        let end = data.len() - bit;

        let mut min = 0;
        let mut min_path: Vec<usize> = vec![];
        let mut tiles: HashMap<usize, Tile> = HashMap::new();
        let mut map_binary: Vec<u8> = vec![EMPTY; map_width * map_height];

        println!("Start: {start}, end: {end}");
        build_map(data, map_width, start, end, &mut tiles, &mut map_binary);
        print_map(&map_binary, map_width);

        walk_graph(
            &mut min,
            &mut min_path,
            [].to_vec(),
            start_index,
            end_index,
            &mut tiles,
            &map_width,
        );

        if min != 0 {
            println!("Found solution; prev: {prev}, bit: {bit}");
            return prev;
        } else {
            println!("No solution so far");
        }

        prev = data.get(end - 1).unwrap().to_string();
    }

    println!("Solution not found?");

    // now try to find last index at which we stil have chance to find way

    "".to_string()
}

fn walk_graph(
    min: &mut usize,
    min_path: &mut Vec<usize>,
    cur_path: Vec<usize>,
    start: usize,
    end: usize,
    tiles: &mut HashMap<usize, Tile>,
    map_width: &usize,
) {
    let tile = tiles.get(&start).unwrap();
    let cost = tile.cost;

    if start == end {
        if *min > 0 {
            if *min > cost {
                *min = cost;
                *min_path = cur_path.to_vec();
            }
        } else {
            *min = cost;
        }
        return;
    }

    let tile_right = tile.right;
    let tile_left = tile.left;
    let tile_up = tile.up;
    let tile_down = tile.down;

    let mut explore = |new_index: usize| {
        let new_cost = cost + 1;
        let next = tiles.get_mut(&new_index).unwrap();
        let rule1 = (next.cost > new_cost && (new_cost < *min || *min == 0)) || next.cost == 0;
        let mut new_path = cur_path.clone();
        new_path.push(new_index);
        if rule1 {
            next.cost = new_cost;
            walk_graph(min, min_path, new_path, new_index, end, tiles, map_width);
        }
    };

    if tile_right {
        explore(start + 1);
    }
    if tile_left {
        explore(start - 1);
    }
    if tile_up {
        explore(start - map_width);
    }
    if tile_down {
        explore(start + map_width);
    }
}

fn build_graph(map_binary: &[u8], map_width: &usize, tiles: &mut HashMap<usize, Tile>) {
    for (index, value) in map_binary.iter().enumerate() {
        if *value != WALL {
            let (x, y) = index_to_xy(&index, map_width);

            let mut tile = Tile::new(
                x,
                y,
                x == 0 && y == 0,
                x == *map_width - 1 && y == *map_width - 1,
            );
            // look around
            if let Some(index) = get_neighbour(index, Direction::East, map_binary, map_width) {
                let (x1, y1) = index_to_xy(&index, map_width);
                tile.link_or_create(x1, y1);
            }
            if let Some(index) = get_neighbour(index, Direction::West, map_binary, map_width) {
                let (x1, y1) = index_to_xy(&index, map_width);
                tile.link_or_create(x1, y1);
            }
            if let Some(index) = get_neighbour(index, Direction::North, map_binary, map_width) {
                let (x1, y1) = index_to_xy(&index, map_width);
                tile.link_or_create(x1, y1);
            }
            if let Some(index) = get_neighbour(index, Direction::South, map_binary, map_width) {
                let (x1, y1) = index_to_xy(&index, map_width);
                tile.link_or_create(x1, y1);
            }
            tiles.insert(index, tile);
        }
    }
}

fn print_map(map: &[u8], map_width: usize) {
    for (index, val) in map.iter().enumerate() {
        if index % map_width == 0 {
            println!();
        }
        if *val == WALL {
            print!("#");
        } else {
            print!(".");
        }
    }
}
fn get_neighbour(
    index: usize,
    direction: Direction,
    map_binary: &[u8],
    map_width: &usize,
) -> Option<usize> {
    let (x, y) = index_to_xy(&index, map_width);

    let mut x1: isize = x as isize;
    let mut y1: isize = y as isize;

    match direction {
        Direction::East => x1 -= 1,
        Direction::West => x1 += 1,
        Direction::North => y1 -= 1,
        Direction::South => y1 += 1,
    };

    if x1 >= *map_width as isize || x1 < 0 || y1 >= *map_width as isize || y1 < 0 {
        return None;
    }

    let index = (y1 * *map_width as isize + x1) as usize;

    if let Some(val) = map_binary.get(index) {
        if *val == EMPTY {
            return Some(index);
        }
    }
    None
}

fn index_to_xy(index: &usize, map_width: &usize) -> (usize, usize) {
    (index % map_width, index / map_width)
}

#[test]
fn test_1() {
    assert_eq!(analyze("test.txt", 7, 7, 12), "6,1");
    //assert_eq!(analyze("input.txt"), 88468);
}
