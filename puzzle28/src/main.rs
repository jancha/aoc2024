use std::fs;

fn main() {
    println!("{}", analyze("input.txt", 101, 103));
}

fn analyze(file: &str, map_width: isize, map_height: isize) -> usize {
    let file = fs::read_to_string(file).expect("Could not read file?");
    let map: Vec<&str> = file.trim().split("\n").collect();

    let mut robots: Vec<(isize, isize, isize, isize)> = vec![];

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
        robots.push((
            *p.first().unwrap(),
            *p.last().unwrap(),
            *v.first().unwrap(),
            *v.last().unwrap(),
        ));
    }

    draw_until_find_tree(&mut robots, map_width, map_height)
}

fn draw_until_find_tree(
    robots: &mut [(isize, isize, isize, isize)],
    map_width: isize,
    map_height: isize,
) -> usize {
    let len = robots.len();

    let mut seconds = 0;

    loop {
        seconds += 1;
        let mut screen: [usize; 10403] = [0; 10403];

        for i in 0..len {
            let robot = robots[i];
            let mut tx = (robot.0 + seconds * robot.2) % map_width;
            if tx < 0 {
                tx += map_width;
            }
            let mut ty = (robot.1 + seconds * robot.3) % map_height;
            if ty < 0 {
                ty += map_height;
            }
            screen[(ty * map_width + tx) as usize] = 1;
        }

        let mut prev = 0;
        let mut ccount = 0;
        let mut new_max_cont = 0;

        for (index, value) in screen.iter().enumerate() {
            if index > 0 {
                if *value != prev {
                    new_max_cont = new_max_cont.max(ccount);
                    ccount = 0;
                } else if *value == 1 {
                    ccount += 1;
                }
            }
            prev = *value;
        }
        if new_max_cont > 10 {
            break;
        }
    }
    seconds as usize
}

#[test]
fn test_1() {
    assert_eq!(analyze("input.txt", 101, 130), 0);
}
