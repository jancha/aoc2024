use std::fs;
fn main() {
    println!("{}", analyze("input.txt"));
}

const PLAYER: u8 = 64;
const WALL: u8 = 35;
const BOX: u8 = 79;
const EMPTY: u8 = 46;

const MOVE_LEFT: u8 = 60;
const MOVE_RIGHT: u8 = 62;
const MOVE_UP: u8 = 94;
const MOVE_DOWN: u8 = 118;

fn analyze(file: &str) -> usize {
    let file = fs::read_to_string(file).expect("Could not read file?");
    let data: Vec<&str> = file.trim().split("\n").map(|x| x.trim()).collect();
    let mut map: Vec<&str> = vec![];
    let mut instructions = String::new();

    let mut stage = 0;
    for i in data {
        if i.is_empty() {
            stage += 1;
            continue;
        }
        if stage == 0 {
            map.push(i);
        } else {
            instructions.push_str(i);
        }
    }

    let map_width = map.first().unwrap().len();

    let mut map_binary: Vec<u8> = map.join("").into();

    let instructions = instructions.as_bytes();

    let (mut pos, _char) = map_binary
        .iter()
        .enumerate()
        .find(|(_x, y)| **y == PLAYER)
        .unwrap();
    for i in instructions {
        move_player(&mut pos, i, &mut map_binary, &map_width);
    }

    let gps: Vec<usize> = map_binary
        .iter()
        .enumerate()
        .map(|(i, v)| {
            if *v == BOX {
                let coords = index_to_xy(&i, &map_width);
                coords.0 + coords.1 * 100
            } else {
                0
            }
        })
        .collect();

    gps.iter().sum()
}

fn move_player(pos: &mut usize, instruction: &u8, map_binary: &mut [u8], map_width: &usize) {
    let new_pos = match *instruction {
        MOVE_LEFT => try_move(*pos as isize - 1, map_binary, map_width, MOVE_LEFT),
        MOVE_RIGHT => try_move(*pos as isize + 1, map_binary, map_width, MOVE_RIGHT),
        MOVE_UP => try_move(
            *pos as isize - *map_width as isize,
            map_binary,
            map_width,
            MOVE_UP,
        ),
        MOVE_DOWN => try_move(
            *pos as isize + *map_width as isize,
            map_binary,
            map_width,
            MOVE_DOWN,
        ),
        _ => panic!("Invalid move"),
    };

    if let Some(new_pos) = new_pos {
        map_binary[*pos] = EMPTY;
        *pos = new_pos as usize;
        map_binary[*pos] = PLAYER;
    }
}

fn try_move(
    new_pos: isize,
    map_binary: &mut [u8],
    map_width: &usize,
    direction: u8,
) -> Option<isize> {
    if !in_bounds(new_pos, map_binary) {
        return None;
    }

    if map_binary[new_pos as usize] == WALL {
        return None; // cannot go there
    }
    if map_binary[new_pos as usize] == BOX
        && try_move_box(new_pos, map_binary, map_width, direction).is_err()
    {
        return None;
    }

    Some(new_pos)
}

fn in_bounds(pos: isize, map_binary: &[u8]) -> bool {
    if pos < 0 {
        false
    } else {
        pos < map_binary.len() as isize
    }
}

fn try_move_box(
    pos: isize,
    map_binary: &mut [u8],
    map_width: &usize,
    direction: u8,
) -> Result<(), ()> {
    let index = match direction {
        MOVE_LEFT => pos - 1,
        MOVE_RIGHT => pos + 1,
        MOVE_UP => pos - *map_width as isize,
        MOVE_DOWN => pos + *map_width as isize,
        _ => panic!("Invalid direction"),
    };

    if !in_bounds(index, map_binary) {
        return Err(());
    }

    let map_index = index as usize;

    if map_binary[map_index] == EMPTY
        || (map_binary[map_index] == BOX
            && try_move_box(index, map_binary, map_width, direction).is_ok())
    {
        map_binary[map_index] = BOX;
        map_binary[pos as usize] = EMPTY;
        return Ok(());
    }

    Err(())
}

fn print_map(map_binary: &[u8], map_width: &usize) {
    for (i, v) in map_binary.iter().enumerate() {
        if i % *map_width == 0 {
            println!("");
        }
        if *v == PLAYER {
            print!("@");
        } else if *v == WALL {
            print!("#");
        } else if *v == BOX {
            print!("O");
        } else {
            print!(".");
        }
    }
    println!("");
}

fn index_to_xy(index: &usize, map_width: &usize) -> (usize, usize) {
    (index % map_width, index / map_width)
}

#[test]
fn test_1() {
    assert_eq!(analyze("test.txt"), 9021);
    //   assert_eq!(analyze("input.txt"), 1465152);
}
