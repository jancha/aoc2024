use std::collections::{HashMap, HashSet};
use std::fs;
fn main() {
    println!("{}", analyze("input.txt"));
}

const START: u8 = 83;
const WALL: u8 = 35;
const EMPTY: u8 = 46;
const END: u8 = 69;

#[derive(Debug, PartialEq)]
enum Direction {
    East,
    West,
    North,
    South,
}

impl Direction {
    fn get_cost(&self, other: &Direction) -> usize {
        match self {
            Direction::East => match other {
                Direction::East => 1,
                Direction::West => 2002,
                Direction::North => 1001,
                Direction::South => 1001,
            },
            Direction::West => match other {
                Direction::East => 2002,
                Direction::West => 1,
                Direction::North => 1001,
                Direction::South => 1001,
            },
            Direction::North => match other {
                Direction::East => 1001,
                Direction::West => 1001,
                Direction::North => 1,
                Direction::South => 2001,
            },
            Direction::South => match other {
                Direction::East => 1001,
                Direction::West => 1001,
                Direction::North => 2001,
                Direction::South => 1,
            },
        }
    }
}

#[derive(Debug, PartialEq)]
struct Tile {
    start: bool,
    finish: bool,
    cost: usize,
    cost_from_west: usize,
    cost_from_east: usize,
    cost_from_south: usize,
    cost_from_north: usize,
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
            cost_from_west: 0,
            cost_from_east: 0,
            cost_from_south: 0,
            cost_from_north: 0,
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

fn analyze(file: &str) -> usize {
    let file = fs::read_to_string(file).expect("Could not read file?");
    let data: Vec<&str> = file.trim().split("\n").map(|x| x.trim()).collect();
    let mut map_binary: Vec<u8> = vec![];

    let mut map_width = 0;
    for i in data {
        if i.is_empty() {
            continue;
        }
        map_width = map_width.max(i.len());
        for c in i.trim().as_bytes() {
            match *c {
                WALL => {
                    map_binary.push(WALL);
                }
                START => {
                    map_binary.push(START);
                }
                END => {
                    map_binary.push(END);
                }
                EMPTY => {
                    map_binary.push(EMPTY);
                }
                _ => {
                    // unsupported char
                }
            }
        }
    }

    let mut tiles: HashMap<usize, Tile> = HashMap::new();

    build_graph(&map_binary, &map_width, &mut tiles);

    // now perform path cost analysis, starting from the start node

    let (start_index, _start) = tiles.iter().find(|(_index, value)| value.start).unwrap();
    let (end_index, _end) = tiles.iter().find(|(_index, value)| value.finish).unwrap();

    let mut min = 0;

    let mut steps: Vec<usize> = vec![];

    let mut solutions: Vec<(Vec<usize>, usize)> = vec![];

    let mut unique = HashSet::new();
    walk_graph(
        &mut min,
        *start_index,
        *end_index,
        &mut tiles,
        &map_width,
        Direction::East,
        &mut steps,
        &mut solutions,
    );
    for (path, _cost) in solutions.into_iter().filter(|(_path, cost)| *cost == min) {
        for i in path {
            if !unique.contains(&i) {
                unique.insert(i);
            }
        }
    }
    unique.len()
}

fn walk_graph(
    min: &mut usize,
    start: usize,
    end: usize,
    tiles: &mut HashMap<usize, Tile>,
    map_width: &usize,
    direction: Direction,
    steps: &mut Vec<usize>,
    solutions: &mut Vec<(Vec<usize>, usize)>,
) {
    let tile = tiles.get(&start).unwrap();
    let cost = tile.cost;

    steps.push(start);

    if start == end {
        if *min > 0 {
            *min = cost.min(*min);
        } else {
            *min = cost;
        }

        if cost == *min {
            let result_steps = steps.to_vec();
            solutions.push((result_steps, cost));
        }
        steps.remove(steps.len() - 1);
        return;
    }

    let tile_right = tile.right;
    let tile_left = tile.left;
    let tile_up = tile.up;
    let tile_down = tile.down;

    let linked_tile_right_cost = if !tile_right {
        None
    } else {
        tiles.get(&(start + 2)).map(|next| next.cost)
    };

    let linked_tile_left_cost = if !tile_left {
        None
    } else {
        tiles.get(&(start - 2)).map(|next| next.cost)
    };

    let linked_tile_up_cost = if !tile_up {
        None
    } else {
        tiles.get(&(start - 2 * *map_width)).map(|next| next.cost)
    };

    let linked_tile_down_cost = if !tile_down {
        None
    } else {
        tiles.get(&(start + 2 * *map_width)).map(|next| next.cost)
    };

    let mut explore = |new_index: usize, new_direction: Direction, linked_tile_cost| {
        let step_cost = direction.get_cost(&new_direction);
        let new_cost = cost + step_cost;

        let rule2 = if let Some(linked_tile_cost) = linked_tile_cost {
            linked_tile_cost >= new_cost + 1
        } else {
            false
        };
        let next = tiles.get_mut(&new_index).unwrap();

        let rule1 = next.cost >= new_cost || next.cost == 0;

        if rule1 || rule2 {
            next.cost = new_cost;
            walk_graph(
                min,
                new_index,
                end,
                tiles,
                map_width,
                new_direction,
                steps,
                solutions,
            )
        }
    };

    if tile_right {
        explore(start + 1, Direction::East, linked_tile_right_cost);
    }
    if tile_left {
        explore(start - 1, Direction::West, linked_tile_left_cost)
    }
    if tile_up {
        explore(start - map_width, Direction::North, linked_tile_up_cost);
    }
    if tile_down {
        explore(start + map_width, Direction::South, linked_tile_down_cost);
    }
    steps.remove(steps.len() - 1);
}

fn build_graph(map_binary: &[u8], map_width: &usize, tiles: &mut HashMap<usize, Tile>) {
    for (index, value) in map_binary.iter().enumerate() {
        if *value != WALL {
            let (x, y) = index_to_xy(&index, map_width);

            let mut tile = Tile::new(x, y, *value == START, *value == END);
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

fn get_neighbour(
    index: usize,
    direction: Direction,
    map_binary: &[u8],
    map_width: &usize,
) -> Option<usize> {
    let delta = match direction {
        Direction::East => 1,
        Direction::West => -1,
        Direction::North => -(*map_width as isize),
        Direction::South => *map_width as isize,
    };

    let new_index = index as isize + delta;
    if new_index < 0 || new_index >= map_binary.len() as isize {
        return None;
    }

    let val = map_binary[new_index as usize];

    if val != WALL {
        Some(new_index as usize)
    } else {
        None
    }
}

fn index_to_xy(index: &usize, map_width: &usize) -> (usize, usize) {
    (index % map_width, index / map_width)
}

#[test]
fn test_1() {
    assert_eq!(analyze("test.txt"), 45);
    assert_eq!(analyze("test2.txt"), 64);
    //assert_eq!(analyze("input.txt"), 616);
}
