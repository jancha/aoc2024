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

    fn mark(&self, index: usize, trace_binary: &mut [u8]) -> Result<(), ()> {
        // if not visited, mark as visited
        // otherwise return error
        let val = trace_binary[index];

        let bit = match self {
            Direction::Up => 1,
            Direction::Right => 2,
            Direction::Down => 4,
            Direction::Left => 8,
        };

        if val > 15 {
            trace_binary[index] = bit;
        } else {
            if val & bit > 0 {
                return Err(());
            }
            trace_binary[index] = val | bit;
        }
        Ok(())
    }
}

fn trace_path(file: &str) -> usize {
    let file = fs::read_to_string(file).expect("Could not read file?");
    let map: Vec<&str> = file.trim().split("\n").collect();
    let map_width = map.first().unwrap().len();
    let map_height = map.len();
    let map = file.replace("\n", "");
    let map_binary = map.as_bytes();

    let mut loops_found = 0;
    let (start_x, start_y) = find_guard(map_binary, map_width);

    // get initial path
    let mut initial_trace_binary = map.clone().as_bytes().to_vec();
    walk_map(
        map_binary,
        &mut initial_trace_binary,
        map_width,
        map_height,
        start_x,
        start_y,
        (9999, 9999),
    );
    let trace_binary_template = map.clone().as_bytes().to_vec();

    // put obstacles
    for y in 0..map_height {
        for x in 0..map_width {
            if x == start_x && y == start_y {
                continue; // cannot be on the start position
            }
            let pos = xy_to_index(x, y, map_width);
            if initial_trace_binary[pos] > 16 {
                // skip, not on initial path
                continue;
            }
            // clean trace
            let mut trace_binary: Vec<u8> = trace_binary_template.iter().map(|_x| 0).collect();

            if walk_map(
                map_binary,
                &mut trace_binary,
                map_width,
                map_height,
                start_x,
                start_y,
                (x, y),
            ) {
                loops_found += 1;
            }
        }
    }
    loops_found
}

fn walk_map(
    map_binary: &[u8],
    trace_binary: &mut [u8],
    map_width: usize,
    map_height: usize,
    start_x: usize,
    start_y: usize,
    obstructions: (usize, usize),
) -> bool {
    let mut direction = Direction::Up;
    let mut x = start_x;
    let mut y = start_y;

    let mut loop_found = false;

    direction
        .mark(xy_to_index(x, y, map_width), trace_binary)
        .unwrap();
    while let Some((new_x, new_y, new_direction)) = move_guard(
        x,
        y,
        direction,
        map_binary,
        map_width,
        map_height,
        obstructions,
    ) {
        x = new_x;
        y = new_y;
        direction = new_direction;
        if direction
            .mark(xy_to_index(x, y, map_width), trace_binary)
            .is_err()
        {
            loop_found = true;
            break;
        }
    }
    loop_found
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
    obstructions: (usize, usize),
) -> Option<(usize, usize, Direction)> {
    let obstruction_x = obstructions.0;
    let obstruction_y = obstructions.1;
    match direction {
        Direction::Up => {
            if y == 0 {
                return None; // reached outer boundary
            }
            if map_binary[xy_to_index(x, y - 1, map_width)] == BLOCK
                || (x == obstruction_x && y - 1 == obstruction_y)
            {
                Some((x, y, direction.rotate()))
            } else {
                Some((x, y - 1, direction))
            }
        }
        Direction::Down => {
            if y == map_height - 1 {
                return None; // reached outer boundary
            }
            if map_binary[xy_to_index(x, y + 1, map_width)] == BLOCK
                || (x == obstruction_x && y + 1 == obstruction_y)
            {
                Some((x, y, direction.rotate()))
            } else {
                Some((x, y + 1, direction))
            }
        }
        Direction::Right => {
            if x == map_width - 1 {
                return None; //reached outer boundary
            }
            if map_binary[xy_to_index(x + 1, y, map_width)] == BLOCK
                || (x + 1 == obstruction_x && y == obstruction_y)
            {
                Some((x, y, direction.rotate()))
            } else {
                Some((x + 1, y, direction))
            }
        }
        Direction::Left => {
            if x == 0 {
                return None; // reached left boundary
            }
            if map_binary[xy_to_index(x - 1, y, map_width)] == BLOCK
                || (x - 1 == obstruction_x && y == obstruction_y)
            {
                Some((x, y, direction.rotate()))
            } else {
                Some((x - 1, y, direction))
            }
        }
    }
}
#[test]

fn test_1() {
    assert_eq!(trace_path("test.txt"), 6);
    assert_eq!(trace_path("input.txt"), 1812);
}
