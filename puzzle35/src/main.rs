use std::fs;
fn main() {
    println!("{}", analyze("input.txt", 71, 71, 1024));
}

const WALL: u8 = 35;

fn analyze(file: &str, map_width: usize, map_height: usize, read_len: usize) -> usize {
    let file = fs::read_to_string(file).expect("Could not read file?");
    let data: Vec<&str> = file.trim().split("\n").map(|x| x.trim()).collect();
    let start_index = 0;
    let end_index = map_width * map_height - 1;

    let mut map_binary: [u8; 71 * 71] = [0; 71 * 71];
    for i in &data[0..read_len] {
        if i.is_empty() {
            continue;
        }

        let mut line = i.split(",");

        let x: usize = line.next().unwrap().parse().unwrap();
        let y: usize = line.next().unwrap().parse().unwrap();

        map_binary[y * map_width + x] = WALL;
    }

    let mut min = 0;
    let mut map_cost: [usize; 71 * 71] = [0; 71 * 71];

    walk_graph(
        &map_binary,
        &mut map_cost,
        &mut min,
        start_index,
        end_index,
        &map_width,
        &map_height,
    );

    min
}

fn walk_graph(
    map_binary: &[u8; 71 * 71],
    map_cost: &mut [usize; 71 * 71],
    min: &mut usize,
    start: usize,
    end: usize,
    map_width: &usize,
    map_height: &usize,
) {
    let cost = map_cost[start];

    if start == end {
        if *min > 0 {
            if *min > cost {
                *min = cost;
            }
        } else {
            *min = cost;
        }
        return;
    }

    let mut explore = |new_index: usize| {
        let new_cost = cost + 1;
        let next_cost = map_cost[new_index];
        let rule1 = (next_cost > new_cost && (new_cost < *min || *min == 0)) || next_cost == 0;
        if rule1 {
            map_cost[new_index] = new_cost;
            walk_graph(
                map_binary, map_cost, min, new_index, end, map_width, map_height,
            );
        }
    };

    let (x, y) = index_to_xy(&start, map_width);

    let is_tile = |dx: isize, dy: isize| -> Option<usize> {
        if dx != 0 {
            let nx = x as isize + dx;
            if nx < 0 || nx >= *map_width as isize {
                None
            } else {
                let index = ((y * *map_width) as isize + nx) as usize;
                if map_binary[index] != WALL {
                    Some(index)
                } else {
                    None
                }
            }
        } else {
            let ny = y as isize + dy;
            if ny < 0 || ny >= *map_height as isize {
                None
            } else {
                let index = ((ny * *map_width as isize) + x as isize) as usize;
                if map_binary[index] != WALL {
                    Some(index)
                } else {
                    None
                }
            }
        }
    };

    if let Some(index) = is_tile(1, 0) {
        explore(index);
    }
    if let Some(index) = is_tile(-1, 0) {
        explore(index);
    }
    if let Some(index) = is_tile(0, -1) {
        explore(index);
    }
    if let Some(index) = is_tile(0, 1) {
        explore(index);
    }
}

fn index_to_xy(index: &usize, map_width: &usize) -> (usize, usize) {
    (index % map_width, index / map_width)
}

#[test]
fn test_1() {
    assert_eq!(analyze("test.txt", 7, 7, 12), 22);
}
