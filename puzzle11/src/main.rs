use std::fs;

fn main() {
    println!("{}", trace_path("input.txt"));
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn rotate(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

fn trace_path(file: &str) -> usize {
    let file = fs::read_to_string(file).expect("Could not read file?");
    let map: Vec<&str> = file.trim().split("\n").collect();
    let map_width = map.first().unwrap().len();
    let map_height = map.len();
    let map = file.replace("\n", "");
    let map_binary = map.as_bytes();

    let mut trace_binary = map.clone().as_bytes().to_vec();

    let mut direction = Direction::Up;
    let (mut x, mut y) = find_guard(map_binary, map_width);

    trace_binary[xy_to_index(x, y, map_width)] = 88;
    while let Some((new_x, new_y, new_direction)) =
        move_guard(x, y, direction, map_binary, map_width, map_height)
    {
        x = new_x;
        y = new_y;
        direction = new_direction;
        trace_binary[xy_to_index(x, y, map_width)] = 88;
    }
    let trace_iter = trace_binary.iter();

    trace_iter.filter(|x| **x == 88).count()
}

fn index_to_xy(index: usize, map_width: usize) -> (usize, usize) {
    (index % map_width, index / map_width)
}
fn xy_to_index(x: usize, y: usize, map_width: usize) -> usize {
    y * map_width + x
}

fn find_guard(map_binary: &[u8], map_width: usize) -> (usize, usize) {
    let position = map_binary
        .iter()
        .position(|x| *x == 94)
        .expect("Guard not found?");
    index_to_xy(position, map_width)
}

const BLOCK: u8 = 35;

fn move_guard(
    x: usize,
    y: usize,
    direction: Direction,
    map_binary: &[u8],
    map_width: usize,
    map_height: usize,
) -> Option<(usize, usize, Direction)> {
    match direction {
        Direction::Up => {
            if y == 0 {
                return None; // reached outer boundary
            }
            if map_binary[xy_to_index(x, y - 1, map_width)] == BLOCK {
                Some((x, y, direction.rotate()))
            } else {
                Some((x, y - 1, direction))
            }
        }
        Direction::Down => {
            if y == map_height - 1 {
                return None; // reached outer boundary
            }
            if map_binary[xy_to_index(x, y + 1, map_width)] == BLOCK {
                Some((x, y, direction.rotate()))
            } else {
                Some((x, y + 1, direction))
            }
        }
        Direction::Right => {
            if x == map_width - 1 {
                return None; //reached outer boundary
            }
            if map_binary[xy_to_index(x + 1, y, map_width)] == BLOCK {
                Some((x, y, direction.rotate()))
            } else {
                Some((x + 1, y, direction))
            }
        }
        Direction::Left => {
            if x == 0 {
                return None; // reached left boundary
            }
            if map_binary[xy_to_index(x - 1, y, map_width)] == BLOCK {
                Some((x, y, direction.rotate()))
            } else {
                Some((x - 1, y, direction))
            }
        }
    }
}
#[test]

fn test_1() {
    assert_eq!(trace_path("test.txt"), 41);
    assert_eq!(trace_path("input.txt"), 5331);
}
