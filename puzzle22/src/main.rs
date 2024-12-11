use std::fs;

fn main() {
    println!("{}", analyze("input.txt", 75));
}

fn analyze(file: &str, mutations: u8) -> usize {
    let file = fs::read_to_string(file).expect("Could not read file?");
    let map: Vec<&str> = file.trim().split(' ').collect();

    let stones: Vec<usize> = map
        .iter()
        .map(|x| {
            let y: usize = x.parse().unwrap();
            blink(y, mutations - 1)
        })
        .collect();
    stones.iter().sum()
}

fn blink(stone: usize, blinks: u8) -> usize {
    let (stone1, stone2) = match stone {
        0 => (1, None),
        x if (x.ilog(10_usize) + 1) % 2 == 0 => {
            let mid_dividor: usize = 10_usize.pow((x.ilog(10_usize) + 1) / 2);
            let stone1 = x / mid_dividor;
            let stone2 = Some(x - stone1 * mid_dividor);
            (stone1, stone2)
        }
        x => (x * 2024, None),
    };

    let mut stones = 0;

    if blinks > 0 {
        stones += blink(stone1, blinks - 1);
        if let Some(stone) = stone2 {
            stones += blink(stone, blinks - 1);
        }
        stones
    } else if stone2.is_some() {
        2
    } else {
        1
    }
}

#[test]
fn test_1() {
    assert_eq!(analyze("test.txt", 6), 22);
    assert_eq!(analyze("test.txt", 25), 55312);
    assert_eq!(analyze("input.txt", 25), 228668);
    assert_eq!(analyze("input.txt", 75), 0);
}
