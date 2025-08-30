use std::cell::RefCell;
use std::collections::HashMap;
use std::io::stdin;

use std::fs;

const PAD_COUNT: usize = 4;

fn main() {
    println!("{}", analyze("input.txt"));
}

fn analyze(file: &str) -> usize {
    let file = fs::read_to_string(file).expect("Could not read file?");
    let data: Vec<&str> = file.trim().split("\n").map(|x| x.trim()).collect();
    let mut result = 0;
    for i in data {
        result += decode(i);
    }
    println!("Final result: {}", result);
    result
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum PadType {
    Door,
    Robot,
}

impl PadType {
    pub fn get_height(&self) -> usize {
        match self {
            PadType::Door => 4,
            PadType::Robot => 2,
        }
    }
    pub fn is_empty_button(&self, index: u8) -> bool {
        match self {
            PadType::Door => index == 9,
            PadType::Robot => index == 0,
        }
    }
    pub fn char_to_index(&self, char: char) -> u8 {
        match self {
            PadType::Door => match char {
                '0' => 10,
                'A' => 11,
                '1' => 6,
                '2' => 7,
                '3' => 8,

                '4' => 3,
                '5' => 4,
                '6' => 5,

                '7' => 0,
                '8' => 1,
                '9' => 2,
                _ => panic!("Unsupported char: {char}"),
            },
            PadType::Robot => match char {
                '^' => 1,
                'v' => 4,
                '<' => 3,
                '>' => 5,
                'A' => 2,
                _ => panic!("Unsupported char {char}"),
            },
        }
    }
    pub fn is_location_valid(&self, x: i8, y: i8) -> bool {
        if x < 0 || y < 0 || x >= 3 || y >= self.get_height() as i8 {
            return false;
        }
        !self.is_empty_button((y * 3 + x) as u8)
    }
    pub fn dxy_to_char(&self, dx: i8, dy: i8) -> char {
        if dx < 0 {
            '<'
        } else if dx > 0 {
            '>'
        } else if dy < 0 {
            '^'
        } else {
            'v'
        }
    }
}
#[derive(PartialEq, Debug, Clone, Copy)]
enum PadController {
    Robot,
    Human,
}

#[derive(PartialEq, Debug)]
struct Pad {
    pad_type: PadType,
    state: u8,
    managed_by: PadController,
    move_history: HashMap<u8, HashMap<u8, Vec<char>>>,
    order: usize,
}

impl Clone for Pad {
    fn clone(&self) -> Pad {
        Pad {
            pad_type: self.pad_type,
            state: self.state,
            managed_by: self.managed_by,
            move_history: HashMap::new(),
            order: self.order,
        }
    }
}

impl Pad {
    fn new(pad_type: PadType, state: u8, managed_by: PadController, order: usize) -> Pad {
        Pad {
            pad_type,
            state,
            managed_by,
            move_history: HashMap::new(),
            order,
        }
    }
    fn get_path(&mut self, target: u8, pads: &Vec<RefCell<Pad>>, memorize: bool) -> Vec<char> {
        let start = self.state;
        self.move_history.entry(start).or_default();
        let history = self.move_history.get_mut(&start).unwrap();
        let path = if !history.contains_key(&target) || !memorize {
            let cost_map: [u8; 12] = [0; 12];
            let padding = get_padding(self.order);

            let (_cost, paths) = self.find_path(self.state, target, vec![], cost_map, [0, 0], pads);

            //determine the cheapes of the min paths
            println!("{padding} Found paths: {:?}", paths);

            let mut min_len = 2 << 8;
            let mut min_path_id = 0;
            if self.order < PAD_COUNT - 1 {
                let mut pad_states: [u8; PAD_COUNT] = [0; PAD_COUNT];
                for i in self.order + 1..4 {
                    pad_states[i] = pads[i].borrow().state;
                }

                for (i, path) in paths.iter().enumerate() {
                    let mut len = 0;
                    for j in path.iter() {
                        let l1 = get_instructions_len(*j, self.order + 1, pads, false);
                        println!("{padding}Len for char {j}: {l1}");
                        len += l1;
                    }
                    println!("{padding}Total len for path: {} {:?} is {}", i, path, len);
                    if len < min_len {
                        min_len = len;
                        min_path_id = i;
                    }
                    for i in self.order + 1..4 {
                        pads[i].borrow_mut().state = pad_states[i];
                    }
                }
            }

            let mut path = paths[min_path_id].clone();

            // ok, good, now we have all min paths, now for each min path, calculate the stop count
            // coming up and choose the shortext

            path.push('A');

            if memorize {
                let history_mut = self.move_history.get_mut(&start).unwrap();

                history_mut.insert(target, path.clone());
            }
            path
        } else {
            self.move_history
                .get(&start)
                .unwrap()
                .get(&target)
                .unwrap()
                .to_vec()
        };

        path // optimize later
    }
    fn find_path(
        &self,
        start: u8,
        target: u8,
        route: Vec<char>,
        cost_map: [u8; 12],
        prev_dxy: [i8; 2],
        pads: &Vec<RefCell<Pad>>,
    ) -> (Option<usize>, Vec<Vec<char>>) {
        //        println!("[path finder] Looking for path to: {target}");
        let directions: [[i8; 2]; 4] = [[-1, 0], [1, 0], [0, -1], [0, 1]];

        let start_x = start as i8 % 3_i8;
        let start_y = start as i8 / 3_i8;

        let mut cost = cost_map[start as usize];
        let cost_map = if cost == 0 {
            let mut m = cost_map;
            m[start as usize] = 1;
            cost = 1;
            m
        } else {
            cost_map
        };

        let mut min_routes: Vec<Vec<char>> = vec![];
        let mut min_cost: Option<usize> = None;

        if start == target {
            //            println!("{padding} Destination found, path: {:?}", route);
            return (Some(cost as usize), vec![route]);
        } else {
            for i in directions.iter() {
                if self
                    .pad_type
                    .is_location_valid(start_x + i[0], start_y + i[1])
                {
                    //                    println!("Start: {start}, target: {target}, Location {:?} is valid, let's check it out", i);
                    let new_index = (start_x + i[0] + (start_y + i[1]) * 3) as usize;

                    // review step cost calc - last todo
                    // path vec (without history) does not seem to work better
                    let step_cost = 1;
                    /*
                    let step_cost = if i[0] == prev_dxy[0] && i[1] == prev_dxy[1] {
                        1
                    } else {
                        3
                    };*/
                    if cost_map[new_index] == 0 {
                        //                      println!("Cost criteria met, new best path found");
                        let mut child_cost_map = cost_map;
                        child_cost_map[new_index] = cost + step_cost;
                        let mut new_route = route.to_vec();
                        new_route.push(self.pad_type.dxy_to_char(i[0], i[1]));

                        let (new_cost, mut routes_to_dst) = self.find_path(
                            new_index as u8,
                            target,
                            new_route,
                            child_cost_map,
                            *i,
                            pads,
                        );
                        if let Some(new_cost) = new_cost {
                            if let Some(actual_min_cost) = min_cost {
                                match actual_min_cost {
                                    _u if new_cost < actual_min_cost => {
                                        min_cost = Some(new_cost);
                                        min_routes = routes_to_dst;
                                    }
                                    _u if new_cost == actual_min_cost => {
                                        min_routes.append(&mut routes_to_dst);
                                    }
                                    _ => {}
                                }
                            } else {
                                min_cost = Some(new_cost);
                                min_routes = routes_to_dst;
                            }
                        }
                    }
                }
            }
        }
        (min_cost, min_routes)
    }
}

fn decode(line: &str) -> usize {
    let mut num: usize = 0;

    let mut pads: Vec<RefCell<Pad>> = vec![];

    pads.push(RefCell::new(Pad::new(
        PadType::Door,
        11,
        PadController::Robot,
        0,
    )));
    for i in 1..PAD_COUNT - 1 {
        pads.push(RefCell::new(Pad::new(
            PadType::Robot,
            2,
            PadController::Robot,
            i,
        )));
    }
    pads.push(RefCell::new(Pad::new(
        PadType::Robot,
        2,
        PadController::Human,
        PAD_COUNT - 1,
    )));

    let mut instructions_len = 0;
    for (i, c) in line.char_indices() {
        if c.is_numeric() {
            let digit = c.to_digit(10).unwrap() as usize;
            let i_u32: u32 = i.try_into().unwrap();
            num += digit * 10_usize.pow(2 - i_u32);
        }

        let part_len = get_instructions_len(c, 0, &pads, true);
        println!("Numeric part: {num} Len: {part_len}\n\n");

        //       let mut buffer = String::new();
        //      stdin().read_line(&mut buffer).unwrap();

        instructions_len += part_len;
    }

    num * instructions_len
}

fn get_padding(pad: usize) -> String {
    let mut padding = String::from(&format!("Pad {pad}"));
    for _i in 0..pad {
        padding.push_str("    ");
    }
    padding
}

fn get_instructions_len(
    target_char: char,
    pad_id: usize,
    pads: &Vec<RefCell<Pad>>,
    memorize: bool,
) -> usize {
    let mut pad = pads[pad_id].borrow_mut();

    let target = pad.pad_type.char_to_index(target_char);
    let padding = get_padding(pad_id);
    let mut len = 0;
    /*   println!(
        "{padding}Pad: {pad} Target Char: '{target_char}', Current Location: {}, Target Location: {target}, Managed by: {:?}",
        pads[pad].state, pads[pad].managed_by
    );*/

    let start = pad.state;
    if pad.managed_by == PadController::Robot {
        let path = pad.get_path(target, pads, memorize).to_vec();
        println!(
            "{padding}Path to target '{target_char}' ({target}) from {}: {:?}",
            start, path
        );

        for i in path.iter() {
            //           println!("{padding}Getting instructions for next step: '{i}'");
            len += get_instructions_len(*i, pad_id + 1, pads, memorize)
        }

        pad.state = target;

        // now navigate back to A
        // and push it enter
    } else {
        println!("{padding} Human pushed '{target_char}'");
        len = 1
    }

    len
}

#[test]
fn test_1() {
    assert_eq!(analyze("test.txt"), 126384);
}
