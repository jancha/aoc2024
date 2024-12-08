use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs;

fn main() {
    println!("{}", analyze("input.txt"));
}

const CHAR_A: u8 = 65;
const CHAR_Z: u8 = 90;

const CHAR_A_SMALL: u8 = 97;
const CHAR_Z_SMALL: u8 = 122;

const CHAR_0: u8 = 48;
const CHAR_9: u8 = 57;

fn analyze(file: &str) -> usize {
    let file = fs::read_to_string(file).expect("Could not read file?");
    let map: Vec<&str> = file.trim().split("\n").collect();

    let map_width = map.first().unwrap().len() as isize;
    let map_height = map.len() as isize;

    let map = file.replace("\n", "");
    let map_binary = map.as_bytes();

    let mut resonance_map_binary = map.clone().as_bytes().to_vec();

    let mut antennas: HashMap<u8, Vec<(isize, isize)>> = HashMap::new();

    for (i, c) in map_binary.iter().enumerate() {
        let i = i as isize;
        if (CHAR_A..=CHAR_Z).contains(c)
            || (CHAR_A_SMALL..=CHAR_Z_SMALL).contains(c)
            || (CHAR_0..=CHAR_9).contains(c)
        {
            let (x, y) = index_to_xy(&i, &map_width);

            match antennas.entry(*c) {
                Entry::Occupied(entry) => {
                    let e = entry.into_mut();
                    e.push((x, y));
                }
                Entry::Vacant(entry) => {
                    let v = vec![(x, y)];
                    entry.insert(v);
                }
            }
        }
    }

    antennas.into_iter().for_each(|(_x, y)| {
        let len = y.len();
        for i in 0..len {
            let a = y.get(i).unwrap();
            for j in i + 1..len {
                let b = y.get(j).unwrap();
                detect_interference(a, b, &map_width, &map_height, &mut resonance_map_binary);
            }
        }
    });

    resonance_map_binary.into_iter().filter(|x| *x == 1).count()
}

fn detect_interference(
    a: &(isize, isize),
    b: &(isize, isize),
    map_width: &isize,
    map_height: &isize,
    resonance_map_binary: &mut [u8],
) {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;

    let mut iter = 0;
    let mut in_bounds = true;
    while in_bounds {
        let ia_x = a.0 + dx * iter;
        let ia_y = a.1 + dy * iter;

        if ia_x < *map_width && ia_y < *map_height && ia_x >= 0 && ia_y >= 0 {
            resonance_map_binary[xy_to_index(&ia_x, &ia_y, map_width) as usize] = 1;
        } else {
            in_bounds = false;
            continue;
        }
        iter += 1;
    }
    in_bounds = true;
    iter = 0;
    while in_bounds {
        let ib_x = b.0 - dx * iter;
        let ib_y = b.1 - dy * iter;
        if ib_x < *map_width && ib_y < *map_height && ib_x >= 0 && ib_y >= 0 {
            resonance_map_binary[xy_to_index(&ib_x, &ib_y, map_width) as usize] = 1;
        } else {
            in_bounds = false;
            continue;
        }
        iter += 1;
    }
}

fn index_to_xy(index: &isize, map_width: &isize) -> (isize, isize) {
    (index % map_width, index / map_width)
}
fn xy_to_index(x: &isize, y: &isize, map_width: &isize) -> isize {
    y * map_width + x
}

#[test]
fn test_1() {
    assert_eq!(analyze("test.txt"), 34);
    assert_eq!(analyze("input.txt"), 927);
}
