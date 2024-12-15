use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    println!("{}", analyze("input.txt"));
}

fn analyze(file: &str) -> usize {
    let file = fs::read_to_string(file).expect("Could not read file?");
    let map: Vec<&str> = file.trim().split("\n").collect();

    let map_width = map.first().unwrap().len();
    let map_height = map.len();

    let map = file.replace("\n", "");
    let map_binary = map.as_bytes();

    let mut result = 0;

    let mut visited = HashMap::new();

    for (index, c) in map_binary.iter().enumerate() {
        if !visited.contains_key(&index) {
            result += find_price(c, &index, map_binary, &map_width, &map_height, &mut visited);
        }
    }

    result
}

fn find_price(
    c: &u8,
    index: &usize,
    map_binary: &[u8],
    map_width: &usize,
    map_height: &usize,
    visited: &mut HashMap<usize, usize>,
) -> usize {
    let (x, y) = index_to_xy(index, map_width);

    let mut area: HashMap<usize, usize> = HashMap::new();

    explore_area(c, &x, &y, map_binary, map_width, map_height, &mut area);

    let area_size = area.len();

    let sides = get_sides(&area, map_width, map_height);

    area.into_iter().for_each(|(i, _j)| {
        visited.insert(i, 1);
    });

    area_size * sides
}

fn explore_area(
    c: &u8,
    x: &usize,
    y: &usize,
    map_binary: &[u8],
    map_width: &usize,
    map_height: &usize,
    area: &mut HashMap<usize, usize>,
) -> usize {
    let index = xy_to_index(x, y, map_width);

    if area.get(&index).is_some() {
        //already on our map
        return 1; // end of path
    }

    let c_new = map_binary[index];

    if c_new != *c {
        return 0; // not continous;
    }

    area.insert(index, 4);

    let mut neighbours_found = 0;
    if *x > 0 {
        neighbours_found += explore_area(c, &(*x - 1), y, map_binary, map_width, map_height, area);
    }
    if *x < map_width - 1 {
        neighbours_found += explore_area(c, &(*x + 1), y, map_binary, map_width, map_height, area);
    }
    if *y > 0 {
        neighbours_found += explore_area(c, x, &(*y - 1), map_binary, map_width, map_height, area);
    }
    if *y < map_height - 1 {
        neighbours_found += explore_area(c, x, &(*y + 1), map_binary, map_width, map_height, area);
    }
    let r = area.get_mut(&index).unwrap();

    *r -= neighbours_found;

    1
}

fn get_sides(area: &HashMap<usize, usize>, map_width: &usize, map_height: &usize) -> usize {
    let mut sides = 0;
    let mut vectors: Vec<Vector> = vec![];
    let squares = map_width * map_height;

    let has_square = |index: isize| -> bool {
        if index < 0 {
            return false;
        }
        if index > squares as isize {
            return false;
        }

        let index = index as usize;

        area.contains_key(&index)
    };

    area.iter().for_each(|square| {
        let index = *square.0;
        let iindex = index as isize;
        let square_xy = index_to_xy(&index, map_width);
        let perimeter = *square.1;

        if perimeter > 0 {
            // not internal square
            // check if there is a neighbour around
            if square_xy.1 > 0 {
                let check = index - *map_width;
                if !area.contains_key(&check) {
                    vectors.push(Vector::from_index(
                        &index,
                        map_width,
                        Side::Top,
                        has_square(iindex - 1),
                        has_square(iindex + 1),
                    ));
                }
            } else {
                vectors.push(Vector::from_index(
                    &index,
                    map_width,
                    Side::Top,
                    has_square(iindex - 1),
                    has_square(iindex + 1),
                ));
            }
            if square_xy.1 < *map_height - 1 {
                let check = index + *map_width;
                if !area.contains_key(&check) {
                    vectors.push(Vector::from_index(
                        &index,
                        map_width,
                        Side::Bottom,
                        has_square(iindex - 1),
                        has_square(iindex + 1),
                    ));
                }
            } else {
                vectors.push(Vector::from_index(
                    &index,
                    map_width,
                    Side::Bottom,
                    has_square(iindex - 1),
                    has_square(iindex + 1),
                ));
            }
            if square_xy.0 < *map_width - 1 {
                let check = index + 1;
                if !area.contains_key(&check) {
                    vectors.push(Vector::from_index(
                        &index,
                        map_width,
                        Side::Right,
                        has_square(iindex - *map_width as isize),
                        has_square(iindex + *map_width as isize),
                    ));
                }
            } else {
                vectors.push(Vector::from_index(
                    &index,
                    map_width,
                    Side::Right,
                    has_square(iindex - *map_width as isize),
                    has_square(iindex + *map_width as isize),
                ));
            }
            if square_xy.0 > 0 {
                let check = index - 1;
                if !area.contains_key(&check) {
                    vectors.push(Vector::from_index(
                        &index,
                        map_width,
                        Side::Left,
                        has_square(iindex - *map_width as isize),
                        has_square(iindex + *map_width as isize),
                    ));
                }
            } else {
                vectors.push(Vector::from_index(
                    &index,
                    map_width,
                    Side::Left,
                    has_square(iindex - *map_width as isize),
                    has_square(iindex + *map_width as isize),
                ));
            }
        }
    });

    let mut new_vectors = vectors.to_vec();
    loop {
        let len = vectors.len();
        let mut restart = false;
        for i in 0..len {
            let vector = vectors.get(i).unwrap();
            for j in i + 1..len {
                let other_vector = vectors.get(j).unwrap();
                if let Ok(new_vector) = vector.combine(other_vector) {
                    /*                   println!(
                        "Combining {:?} with {:?} into {:?}",
                        vector, other_vector, new_vector,
                    );*/
                    new_vectors.remove(j);
                    new_vectors.remove(i);
                    new_vectors.push(new_vector);
                    restart = true;
                    break;
                }
            }
            if restart {
                break;
            }
        }
        if restart {
            vectors = new_vectors.clone();
        } else {
            break;
        }
    }
    vectors.len()
}
enum Side {
    Top,
    Right,
    Left,
    Bottom,
}

#[derive(PartialEq, Debug, Clone)]
struct Dot {
    x: usize,
    y: usize,
}
impl Dot {
    fn new(x: usize, y: usize) -> Dot {
        Dot { x, y }
    }
}
#[derive(PartialEq, Debug, Clone)]
enum VectorDirection {
    Horiziontal,
    Vertical,
}

#[derive(PartialEq, Debug, Clone)]
struct Vector {
    start: Dot,
    end: Dot,
    direction: VectorDirection,
    can_combine_start: bool,
    can_combine_end: bool,
}

impl Vector {
    fn new(start: Dot, end: Dot, can_combine_start: bool, can_combine_end: bool) -> Vector {
        let direction = if start.x == end.x {
            VectorDirection::Vertical
        } else {
            VectorDirection::Horiziontal
        };
        Vector {
            start,
            end,
            direction,
            can_combine_start,
            can_combine_end,
        }
    }
    fn connects(&self, other: &Vector) -> bool {
        self.start == other.start
            || self.start == other.end
            || self.end == other.start
            || self.end == other.end
    }

    fn may_connect(&self, other: &Vector) -> bool {
        if self.direction == VectorDirection::Horiziontal {
            if self.start.x.min(self.end.x) > other.start.x.min(other.end.x) {
                self.can_combine_start
            } else {
                self.can_combine_end
            }
        } else if self.start.y.min(self.end.y) > other.start.y.min(other.end.y) {
            self.can_combine_start
        } else {
            self.can_combine_end
        }
    }

    fn aligned(&self, other: &Vector) -> bool {
        self.direction == other.direction
    }
    fn combine(&self, other: &Vector) -> Result<Vector, ()> {
        if self.aligned(other) && self.connects(other) && self.may_connect(other) {
            if self.direction == VectorDirection::Vertical {
                let start_y = other
                    .end
                    .y
                    .min(self.end.y.min(self.start.y.min(other.start.y)));

                let can_combine_start = if start_y == self.start.y {
                    self.can_combine_start
                } else {
                    other.can_combine_start
                };

                let end_y = other
                    .end
                    .y
                    .max(self.end.y.max(self.start.y.max(other.start.y)));

                let can_combine_end = if end_y == self.end.y {
                    self.can_combine_end
                } else {
                    other.can_combine_end
                };

                return Ok(Vector::new(
                    Dot::new(self.start.x, start_y),
                    Dot::new(self.start.x, end_y),
                    can_combine_start,
                    can_combine_end,
                ));
            } else {
                let start_x = other
                    .end
                    .x
                    .min(self.end.x.min(self.start.x.min(other.start.x)));

                let can_combine_start = if start_x == self.start.x {
                    self.can_combine_start
                } else {
                    other.can_combine_start
                };

                let end_x = other
                    .end
                    .x
                    .max(self.end.x.max(self.start.x.max(other.start.x)));

                let can_combine_end = if end_x == self.end.x {
                    self.can_combine_end
                } else {
                    other.can_combine_end
                };

                return Ok(Vector::new(
                    Dot::new(start_x, self.start.y),
                    Dot::new(end_x, self.start.y),
                    can_combine_start,
                    can_combine_end,
                ));
            }
        }
        Err(())
    }
    fn from_index(
        index: &usize,
        map_width: &usize,
        side: Side,
        can_combine_start: bool,
        can_combine_end: bool,
    ) -> Vector {
        let square = index_to_xy(index, map_width);
        let (dot1, dot2) = match side {
            Side::Top => (
                Dot::new(square.0, square.1),
                Dot::new(square.0 + 1, square.1),
            ),
            Side::Bottom => (
                Dot::new(square.0, square.1 + 1),
                Dot::new(square.0 + 1, square.1 + 1),
            ),
            Side::Left => (
                Dot::new(square.0, square.1),
                Dot::new(square.0, square.1 + 1),
            ),
            Side::Right => (
                Dot::new(square.0 + 1, square.1),
                Dot::new(square.0 + 1, square.1 + 1),
            ),
        };
        Vector::new(dot1, dot2, can_combine_start, can_combine_end)
    }
}

fn index_to_xy(index: &usize, map_width: &usize) -> (usize, usize) {
    (index % map_width, index / map_width)
}
fn xy_to_index(x: &usize, y: &usize, map_width: &usize) -> usize {
    y * map_width + x
}

#[test]
fn test_1() {
    assert_eq!(analyze("test.txt"), 1206);
    assert_eq!(analyze("test2.txt"), 368);
    assert_eq!(analyze("test3.txt"), 236);
    assert_eq!(analyze("input.txt"), 921636);
}
