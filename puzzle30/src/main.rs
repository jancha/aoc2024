use std::fs;
fn main() {
    println!("{}", analyze("input.txt"));
}

const PLAYER: u8 = 64;
const WALL: u8 = 35;
const BOX: u8 = 79;
const EMPTY: u8 = 46;
const BOX_OPEN: u8 = 91;
const BOX_CLOSE: u8 = 93;

const MOVE_LEFT: u8 = 60;
const MOVE_RIGHT: u8 = 62;
const MOVE_UP: u8 = 94;
const MOVE_DOWN: u8 = 118;

fn analyze(file: &str) -> usize {
    let file = fs::read_to_string(file).expect("Could not read file?");
    let data: Vec<&str> = file.trim().split("\n").map(|x| x.trim()).collect();
    let mut map_binary: Vec<u8> = vec![];
    let mut instructions = String::new();
    let mut stage = 0;

    let mut map_width = 0;
    for i in data {
        if i.is_empty() {
            stage += 1;
            continue;
        }
        if stage == 0 {
            map_width = map_width.max(i.len() * 2);
            for c in i.trim().as_bytes() {
                match *c {
                    WALL => {
                        map_binary.push(WALL);
                        map_binary.push(WALL);
                    }
                    BOX => {
                        map_binary.push(BOX_OPEN);
                        map_binary.push(BOX_CLOSE);
                    }
                    PLAYER => {
                        map_binary.push(PLAYER);
                        map_binary.push(EMPTY);
                    }
                    EMPTY => {
                        map_binary.push(EMPTY);
                        map_binary.push(EMPTY);
                    }
                    _ => {
                        // unsupported char
                    }
                }
            }
        } else {
            instructions.push_str(i);
        }
    }

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
            if *v == BOX_OPEN {
                let coords = index_to_xy(&i, &map_width);
                coords.0 + coords.1 * 100
            } else {
                0
            }
        })
        .collect();

    gps.iter().sum()
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

fn move_player(
    current_index: &mut usize,
    instruction: &u8,
    map_binary: &mut [u8],
    map_width: &usize,
) {
    if let Some(new_index) = move_index(current_index, instruction, map_binary, map_width) {
        if let Some(new_index) = try_move_player(new_index, map_binary, map_width, *instruction) {
            map_binary.swap(*current_index, new_index);
            *current_index = new_index;
        }
    }
}

fn try_move_player(
    new_index: usize,
    map_binary: &mut [u8],
    map_width: &usize,
    direction: u8,
) -> Option<usize> {
    if map_binary[new_index] == WALL {
        return None; // cannot go there
    }
    if (map_binary[new_index] == BOX_OPEN || map_binary[new_index] == BOX_CLOSE)
        && try_move_box(new_index, map_binary, map_width, direction).is_err()
    {
        return None;
    }
    Some(new_index)
}

fn in_bounds(pos: isize, map_binary: &[u8]) -> bool {
    if pos < 0 {
        false
    } else {
        pos < map_binary.len() as isize
    }
}

fn try_move_box(
    box_position: usize,
    map_binary: &mut [u8],
    map_width: &usize,
    direction: u8,
) -> Result<(), ()> {
    // check if above the box is one box or two boxes
    // if two boxes, we need to be able to move both before moving this one

    // ok so, now check if boxes are movable

    if direction == MOVE_LEFT || direction == MOVE_RIGHT {
        if can_move_box_horizontal(box_position, map_binary, map_width, direction) {
            do_move_box_horizontal(box_position, map_binary, map_width, direction)?;
            return Ok(());
        }
    } else if can_move_box(box_position, map_binary, map_width, direction) {
        do_move_box(box_position, map_binary, map_width, direction)?;
        return Ok(());
    }
    Err(())
}

fn can_move_box_horizontal(
    box_position: usize,
    map_binary: &[u8],
    map_width: &usize,
    direction: u8,
) -> bool {
    if let Some(box_part1_new_position) =
        move_index(&box_position, &direction, map_binary, map_width)
    {
        if map_binary[box_part1_new_position] == EMPTY {
            //empty slots for both sides
            return true;
        }

        if map_binary[box_part1_new_position] == WALL {
            // wall detected, cannot move
            return false;
        }

        return can_move_box_horizontal(box_part1_new_position, map_binary, map_width, direction);
    }
    false
}

fn do_move_box_horizontal(
    box_position: usize,
    map_binary: &mut [u8],
    map_width: &usize,
    direction: u8,
) -> Result<(), ()> {
    if let Some(box_part1_new_position) =
        move_index(&box_position, &direction, map_binary, map_width)
    {
        if map_binary[box_part1_new_position] == BOX_CLOSE
            || map_binary[box_part1_new_position] == BOX_OPEN
        {
            do_move_box_horizontal(box_part1_new_position, map_binary, map_width, direction)?;
        }
        map_binary.swap(box_position, box_part1_new_position);
        return Ok(());
    }
    Err(())
}

fn can_move_box(box_position: usize, map_binary: &[u8], map_width: &usize, direction: u8) -> bool {
    let box_part1 = map_binary[box_position];
    let box_position2 =
        (if box_part1 == BOX_OPEN { 1 } else { -1 } + box_position as isize) as usize;

    if let Some(box_part1_new_position) =
        move_index(&box_position, &direction, map_binary, map_width)
    {
        if let Some(box_part2_new_position) =
            move_index(&box_position2, &direction, map_binary, map_width)
        {
            if map_binary[box_part1_new_position] == EMPTY
                && map_binary[box_part2_new_position] == EMPTY
            {
                //empty slots for both sides
                return true;
            }

            if map_binary[box_part1_new_position] == WALL
                || map_binary[box_part2_new_position] == WALL
            {
                // wall detected, cannot move
                return false;
            }

            if map_binary[box_part1_new_position] == box_part1 {
                // only one box needs to be check
                return can_move_box(box_part1_new_position, map_binary, map_width, direction);
            } else {
                // two separate boxes above
                //
                let mut can_move = true;
                if map_binary[box_part1_new_position] == BOX_OPEN
                    || map_binary[box_part1_new_position] == BOX_CLOSE
                {
                    can_move =
                        can_move_box(box_part1_new_position, map_binary, map_width, direction);
                }
                if (map_binary[box_part2_new_position] == BOX_OPEN
                    || map_binary[box_part2_new_position] == BOX_CLOSE)
                    && can_move
                {
                    can_move =
                        can_move_box(box_part2_new_position, map_binary, map_width, direction);
                }
                return can_move;
            }
        }
    }
    false
}

fn do_move_box(
    box_position: usize,
    map_binary: &mut [u8],
    map_width: &usize,
    direction: u8,
) -> Result<(), ()> {
    let box_part1 = map_binary[box_position];
    let box_position2 =
        (if box_part1 == BOX_OPEN { 1 } else { -1 } + box_position as isize) as usize;

    if let Some(box_part1_new_position) =
        move_index(&box_position, &direction, map_binary, map_width)
    {
        if let Some(box_part2_new_position) =
            move_index(&box_position2, &direction, map_binary, map_width)
        {
            if map_binary[box_part1_new_position] == box_part1 {
                // only one box needs to be check
                do_move_box(box_part1_new_position, map_binary, map_width, direction)?;
            } else if map_binary[box_part1_new_position] == BOX_CLOSE
                || map_binary[box_part1_new_position] == BOX_OPEN
            {
                if map_binary[box_part1_new_position] == BOX_OPEN
                    || map_binary[box_part1_new_position] == BOX_CLOSE
                {
                    do_move_box(box_part1_new_position, map_binary, map_width, direction)?;
                }
                if map_binary[box_part2_new_position] == BOX_OPEN
                    || map_binary[box_part2_new_position] == BOX_CLOSE
                {
                    do_move_box(box_part2_new_position, map_binary, map_width, direction)?;
                }
                // two separate boxes above
            } else if map_binary[box_part2_new_position] == BOX_CLOSE
                || map_binary[box_part2_new_position] == BOX_OPEN
            {
                if map_binary[box_part2_new_position] == BOX_OPEN
                    || map_binary[box_part2_new_position] == BOX_CLOSE
                {
                    do_move_box(box_part2_new_position, map_binary, map_width, direction)?;
                }
                if map_binary[box_part2_new_position] == BOX_OPEN
                    || map_binary[box_part2_new_position] == BOX_CLOSE
                {
                    do_move_box(box_part2_new_position, map_binary, map_width, direction)?;
                }
                // two separate boxes above
            }

            map_binary.swap(box_position, box_part1_new_position);
            map_binary.swap(box_position2, box_part2_new_position);
            return Ok(());
        }
    }
    Err(())
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
