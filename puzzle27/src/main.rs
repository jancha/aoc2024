use std::fs;

fn main() {
    println!("{}", analyze("input.txt", 101, 103));
}

fn analyze(file: &str, map_width: isize, map_height: isize) -> usize {
    let file = fs::read_to_string(file).expect("Could not read file?");
    let map: Vec<&str> = file.trim().split("\n").collect();

    let simulation_time = 100;

    let map_half_width = map_width / 2;
    let map_half_height = map_height / 2;

    let mut quadrants = [0, 0, 0, 0];

    for i in map {
        let i: Vec<&str> = i.split(" ").collect();
        let p: Vec<isize> = i
            .first()
            .unwrap()
            .split("=")
            .last()
            .unwrap()
            .split(",")
            .map(|x| {
                let y: isize = x.parse().unwrap();
                y
            })
            .collect();

        let v: Vec<isize> = i
            .last()
            .unwrap()
            .split("=")
            .last()
            .unwrap()
            .split(",")
            .map(|x| {
                let y: isize = x.parse().unwrap();
                y
            })
            .collect();

        let mut tx = (p.first().unwrap() + simulation_time * v.first().unwrap()) % map_width;
        if tx < 0 {
            tx += map_width;
        }
        let mut ty = (p.last().unwrap() + simulation_time * v.last().unwrap()) % map_height;
        if ty < 0 {
            ty += map_height;
        }

        let quadrant = if tx < map_half_width && ty < map_half_height {
            0
        } else if tx < map_half_width && ty >= map_height - map_half_height {
            2
        } else if tx >= map_width - map_half_width && ty < map_half_height {
            1
        } else if tx >= map_width - map_half_width && ty >= map_height - map_half_height {
            3
        } else {
            -1
        };

        if quadrant > -1 {
            quadrants[quadrant as usize] += 1;
        }
    }
    quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3]
}

#[test]
fn test_1() {
    assert_eq!(analyze("test.txt", 11, 7), 12);
    assert_eq!(analyze("input.txt", 101, 130), 211773366);
}
