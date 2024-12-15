use std::fs;

fn main() {
    println!("{}", analyze("input.txt"));
}

fn analyze(file: &str) -> usize {
    let file = fs::read_to_string(file).expect("Could not read file?");
    let map: Vec<&str> = file.trim().split("\n").collect();

    let mut stage = 1;
    let mut a_dx = 0;
    let mut b_dx = 0;
    let mut a_dy = 0;
    let mut b_dy = 0;
    let mut t_x = 0;
    let mut t_y = 0;

    let mut tokens = 0;
    for i in map {
        if i.is_empty() {
            continue;
        }

        let list: Vec<&str> = i.split(", ").collect();

        if stage == 1 {
            a_dx = list
                .first()
                .unwrap()
                .split("+")
                .last()
                .unwrap()
                .parse()
                .unwrap();
            a_dy = list
                .last()
                .unwrap()
                .split("+")
                .last()
                .unwrap()
                .parse()
                .unwrap();
            stage += 1;
        } else if stage == 2 {
            b_dx = list
                .first()
                .unwrap()
                .split("+")
                .last()
                .unwrap()
                .parse()
                .unwrap();
            b_dy = list
                .last()
                .unwrap()
                .split("+")
                .last()
                .unwrap()
                .parse()
                .unwrap();
            stage += 1;
        } else if stage == 3 {
            t_x = list
                .first()
                .unwrap()
                .split("=")
                .last()
                .unwrap()
                .parse()
                .unwrap();
            t_y = list
                .last()
                .unwrap()
                .split("=")
                .last()
                .unwrap()
                .parse()
                .unwrap();
            stage += 1;
        }

        if stage == 4 {
            tokens += solve(a_dx, a_dy, b_dx, b_dy, t_x, t_y);
            stage = 1;
        }
    }

    tokens
}

fn solve(a_dx: usize, a_dy: usize, b_dx: usize, b_dy: usize, t_x: usize, t_y: usize) -> usize {
    println!(
        "Adx {} Ady {} Bdx {} Bdy {} Tx {} Ty {}",
        a_dx, a_dy, b_dx, b_dy, t_x, t_y
    );
    let b = (t_x as isize * a_dy as isize - a_dx as isize * t_y as isize)
        / (a_dy as isize * b_dx as isize - b_dy as isize * a_dx as isize);

    println!("B: {}", b);

    if b < 0 || b > 100 {
        return 0;
    }

    let a = (t_y as isize - b_dy as isize * b) / a_dy as isize;

    println!("A: {}", a);
    if a > 100 || a < 0 {
        return 0;
    }

    if a_dx * a as usize + b_dx * b as usize != t_x || a_dy * a as usize + b_dy * b as usize != t_y
    {
        println!("Solution not found");
        return 0;
    }

    (a * 3 + b) as usize
}

#[test]
fn test_1() {
    assert_eq!(analyze("test.txt"), 480);
    //   assert_eq!(analyze("input.txt"), 0);
}
