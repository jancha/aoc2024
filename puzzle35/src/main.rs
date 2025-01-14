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

fn analyze(file: &str, map_width: usize, map_height: usize, read_len: usize) -> usize {
    let file = fs::read_to_string(file).expect("Could not read file?");
    let data: Vec<&str> = file.trim().split("\n").map(|x| x.trim()).collect();
    let mut map_binary: Vec<u8> = vec![EMPTY; map_width * map_height];

    for i in &data[0..read_len] {
        if i.is_empty() {
            continue;
        }

        let mut line = i.split(",");

        let x: usize = line.next().unwrap().parse().unwrap();
        let y: usize = line.next().unwrap().parse().unwrap();

        map_binary[y * map_width + x] = WALL;
    }

    print_map(&map_binary, map_width);

    let mut tiles: HashMap<usize, Tile> = HashMap::new();

    build_graph(&map_binary, &map_width, &mut tiles);

    // now perform path cost analysis, starting from the start node

    let start_index = 0;
    let end_index = map_width * map_height - 1;

    let mut min = 0;

    walk_graph(&mut min, start_index, end_index, &mut tiles, &map_width);

    min
}

fn walk_graph(
    min: &mut usize,
    start: usize,
    end: usize,
    tiles: &mut HashMap<usize, Tile>,
    map_width: &usize,
) {
    let tile = tiles.get(&start).unwrap();
    let cost = tile.cost;

    if start == end {
        if *min > 0 {
            *min = cost.min(*min);
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
        if rule1 {
            next.cost = new_cost;
            walk_graph(min, new_index, end, tiles, map_width);
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
    assert_eq!(analyze("test.txt", 7, 7, 12), 22);
    //assert_eq!(analyze("input.txt"), 88468);
}
