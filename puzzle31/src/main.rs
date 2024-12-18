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
    fn link_or_create(&mut self, x: usize, y: usize, tile: &u8, tiles: &mut Vec<Tile>) {
        if let Some(tile) = tiles.iter_mut().find(|tile| tile.x == x && tile.y == y) {
            self.link(x, y, tile);
        } else {
            let mut tile = Tile::new(x, y, *tile == START, *tile == END);
            self.link(x, y, &mut tile);
            tiles.push(tile);
        }
    }
    fn link(&mut self, x: usize, y: usize, tile: &mut Tile) {
        if x < self.x {
            self.left = true;
            tile.right = true;
        } else if x > self.x {
            self.right = true;
            tile.left = true;
        } else if y < self.y {
            self.up = true;
            tile.down = true;
        } else {
            self.down = true;
            tile.up = true;
        }
    }
    fn get_left<'a>(&self, tiles: &'a Vec<Tile>) -> Option<&'a Tile> {
        if !self.left {
            return None;
        }
        tiles
            .iter()
            .find(|tile| tile.x == self.x - 1 && tile.y == self.y)
    }
    fn get_right<'a>(&self, tiles: &'a Vec<Tile>) -> Option<&'a Tile> {
        if !self.right {
            return None;
        }
        tiles
            .iter()
            .find(|tile| tile.x == self.x + 1 && tile.y == self.y)
    }

    fn get_up<'a>(&self, tiles: &'a Vec<Tile>) -> Option<&'a Tile> {
        if !self.up {
            return None;
        }
        tiles
            .iter()
            .find(|tile| tile.x == self.x && tile.y == self.y - 1)
    }

    fn get_down<'a>(&self, tiles: &'a Vec<Tile>) -> Option<&'a Tile> {
        if !self.down {
            return None;
        }
        tiles
            .iter()
            .find(|tile| tile.x == self.x && tile.y == self.y + 1)
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

    let mut tiles: Vec<Tile> = vec![];

    build_graph(&map_binary, &map_width, &mut tiles);

    // now perform path cost analysis, starting from the start node

    let start = tiles.iter().find(|x| x.start).unwrap();
    let end = tiles.iter().find(|x| x.finish).unwrap();
    println!("Start {:?}, End {:?}", &start, &end);

    let cost = walk_graph(
        start.x,
        start.y,
        end.x,
        end.y,
        &mut tiles,
        Direction::East,
        0,
    );

    println!("Cost: {:?}", cost);

    cost.unwrap()
}

fn walk_graph(
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
    tiles: &mut Vec<Tile>,
    direction: Direction,
    level: usize,
) -> Option<usize> {
    let start = tiles
        .iter()
        .find(|tile| tile.x == start_x && tile.y == start_y)
        .unwrap();

    if start_x == end_x && start_y == end_y {
        println!("Found end: {} {}", start.cost, level);
        // panic!();
        return Some(start.cost);
    }

    //    println!("level: {}", level);

    let cost = start.cost;
    let start_right = start.right;
    let start_left = start.left;
    let start_up = start.up;
    let start_down = start.down;

    let mut paths = vec![];
    if start_right {
        let step_cost = if direction == Direction::North || direction == Direction::South {
            1000
        } else if direction == Direction::West {
            2000
        } else {
            0
        } + 1;
        //        println!("Right step cost: {} {} {}", start_x, start_y, step_cost);
        let next = tiles
            .iter_mut()
            .find(|tile| tile.x == start_x + 1 && tile.y == start_y)
            .unwrap();

        if next.cost >= cost + step_cost || next.cost == 0 {
            next.cost = cost + step_cost;
            if let Some(cost) = walk_graph(
                next.x,
                next.y,
                end_x,
                end_y,
                tiles,
                Direction::East,
                level + 1,
            ) {
                paths.push(cost);
            }
        }
    }
    if start_left {
        let step_cost = if direction == Direction::North || direction == Direction::South {
            1000
        } else if direction == Direction::East {
            2000
        } else {
            0
        } + 1;
        //      println!("Left step cost: {} {} {}", start_x, start_y, step_cost);
        let next = tiles
            .iter_mut()
            .find(|tile| tile.x == start_x - 1 && tile.y == start_y)
            .unwrap();

        if next.cost >= cost + step_cost || next.cost == 0 {
            next.cost = cost + step_cost;
            if let Some(cost) = walk_graph(
                next.x,
                next.y,
                end_x,
                end_y,
                tiles,
                Direction::West,
                level + 1,
            ) {
                paths.push(cost);
            }
        }
    }
    if start_up {
        let step_cost = if direction == Direction::West || direction == Direction::East {
            1000
        } else if direction == Direction::South {
            2000
        } else {
            0
        } + 1;
        //    println!("Up step cost: {} {} {}", start_x, start_y, step_cost);
        let next = tiles
            .iter_mut()
            .find(|tile| tile.x == start_x && tile.y == start_y - 1)
            .unwrap();

        if next.cost >= cost + step_cost || next.cost == 0 {
            next.cost = cost + step_cost;
            if let Some(cost) = walk_graph(
                next.x,
                next.y,
                end_x,
                end_y,
                tiles,
                Direction::North,
                level + 1,
            ) {
                paths.push(cost);
            }
        }
    }
    if start_down {
        let step_cost = if direction == Direction::West || direction == Direction::East {
            1000
        } else if direction == Direction::North {
            2000
        } else {
            0
        } + 1;
        //  println!("Down step cost: {} {} {}", start_x, start_y, step_cost);
        let next = tiles
            .iter_mut()
            .find(|tile| tile.x == start_x && tile.y == start_y + 1)
            .unwrap();

        if next.cost >= cost + step_cost || next.cost == 0 {
            next.cost = cost + step_cost;
            if let Some(cost) = walk_graph(
                next.x,
                next.y,
                end_x,
                end_y,
                tiles,
                Direction::South,
                level + 1,
            ) {
                paths.push(cost);
            }
        }
    }

    if paths.is_empty() {
        None
    } else {
        //println!("Paths from: {} {}  {:?}", start_x, start_y, paths);
        let min = paths.iter().min().unwrap();
        Some(*min)
    }
}

fn build_graph(map_binary: &[u8], map_width: &usize, tiles: &mut Vec<Tile>) {
    for (index, value) in map_binary.iter().enumerate() {
        if *value != WALL {
            let (x, y) = index_to_xy(&index, map_width);

            if !tiles.iter().any(|tile| tile.x == x && tile.y == y) {
                let mut tile = Tile::new(x, y, *value == START, *value == END);

                // look around
                if let Some((index, other_tile)) =
                    get_neighbour(index, Direction::East, map_binary, map_width)
                {
                    let (x1, y1) = index_to_xy(&index, map_width);
                    tile.link_or_create(x1, y1, &other_tile, tiles);
                }
                if let Some((index, other_tile)) =
                    get_neighbour(index, Direction::West, map_binary, map_width)
                {
                    let (x1, y1) = index_to_xy(&index, map_width);
                    tile.link_or_create(x1, y1, &other_tile, tiles);
                }
                if let Some((index, other_tile)) =
                    get_neighbour(index, Direction::North, map_binary, map_width)
                {
                    let (x1, y1) = index_to_xy(&index, map_width);
                    tile.link_or_create(x1, y1, &other_tile, tiles);
                }
                if let Some((index, other_tile)) =
                    get_neighbour(index, Direction::South, map_binary, map_width)
                {
                    let (x1, y1) = index_to_xy(&index, map_width);
                    tile.link_or_create(x1, y1, &other_tile, tiles);
                }

                tiles.push(tile);
            }
        }
    }
}

fn get_neighbour(
    index: usize,
    direction: Direction,
    map_binary: &[u8],
    map_width: &usize,
) -> Option<(usize, u8)> {
    let delta = match direction {
        Direction::East => 1,
        Direction::West => -1,
        Direction::North => -(*map_width as isize),
        Direction::South => *map_width as isize,
    } as isize;

    let new_index = index as isize + delta;
    if new_index < 0 || new_index >= map_binary.len() as isize {
        return None;
    }

    let val = map_binary[new_index as usize];

    if val != WALL {
        Some((new_index as usize, val))
    } else {
        None
    }
}

fn index_to_xy(index: &usize, map_width: &usize) -> (usize, usize) {
    (index % map_width, index / map_width)
}

/*fn move_index(
    current_index: &usize,
    instruction: &u8,
    map_binary: &[u8],
    map_width: &usize,
) -> Option<usize> {
    let new_index = match *instruction {
        MOVE_LEFT => -1,
        MOVE_RIGHT => 1,
        MOVE_UP => -(*map_width as isize),
        MOVE_DOWN => *map_width as isize,
        _ => panic!("Invalid move"),
    } + *current_index as isize;

    if !in_bounds(new_index, map_binary) {
        return None;
    }
    Some(new_index as usize)
}

fn print_map(map_binary: &[u8], map_width: &usize) {
    for (i, v) in map_binary.iter().enumerate() {
        if i % *map_width == 0 {
            println!("");
        }
        print!("{}", *v as char);
    }
    println!("");
}*/
#[test]
fn test_1() {
    assert_eq!(analyze("test.txt"), 7036);
    assert_eq!(analyze("test2.txt"), 11048);
    assert_eq!(analyze("input.txt"), 1465152);
}
