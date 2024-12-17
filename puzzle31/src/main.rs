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
struct Tile<'a> {
    start: bool,
    finish: bool,
    cost: usize,
    left: Option<&'a Tile<'a>>,
    right: Option<&'a Tile<'a>>,
    up: Option<&'a Tile<'a>>,
    down: Option<&'a Tile<'a>>,
    x: usize,
    y: usize,
}

impl<'a> Tile<'a> {
    fn new(x: usize, y: usize, start: bool, finish: bool) -> Tile<'a> {
        Tile {
            start,
            finish,
            cost: 0,
            left: None,
            right: None,
            up: None,
            down: None,
            x,
            y,
        }
    }
}

fn analyze(file: &str) -> usize {
    let file = fs::read_to_string(file).expect("Could not read file?");
    let data: Vec<&str> = file.trim().split("\n").map(|x| x.trim()).collect();
    let mut map_binary: Vec<u8> = vec![];
    let mut instructions = String::new();
    let mut stage = 0;

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
    0
}

fn move_index(
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
}

fn index_to_xy(index: &usize, map_width: &usize) -> (usize, usize) {
    (index % map_width, index / map_width)
}

#[test]
fn test_1() {
    //assert_eq!(analyze("test.txt"), 618);
    assert_eq!(analyze("test2.txt"), 9021);
    //   assert_eq!(analyze("input.txt"), 1465152);
}
