use std::fs;

fn main() {
    println!("{}", analyze("input.txt", 25));
}

fn analyze(file: &str, mutations: u8) -> usize {
    let file = fs::read_to_string(file).expect("Could not read file?");
    let map: Vec<&str> = file.trim().split(' ').collect();

    let mut stones: Vec<usize> = map
        .iter()
        .map(|x| {
            let y: usize = x.parse().unwrap();
            y
        })
        .collect();

    for _i in 0..mutations {
        let mut stones_new: Vec<usize> = vec![];

        let len = stones.len();

        for j in 0..len {
            let stone = stones.get(j).unwrap();

            match stone {
                0 => stones_new.push(1),
                x if (x.ilog(10_usize) + 1) % 2 == 0 => {
                    let mid_dividor: usize = 10_usize.pow((x.ilog(10_usize) + 1) / 2);
                    let left = x / mid_dividor;
                    let right = x - left * mid_dividor;
                    stones_new.push(left);
                    stones_new.push(right);
                }
                x => {
                    stones_new.push(x * 2024);
                }
            }
        }
        stones = stones_new;
    }
    stones.len()
}

#[test]
fn test_1() {
    assert_eq!(analyze("test.txt", 6), 22);
    assert_eq!(analyze("test.txt", 25), 55312);
    assert_eq!(analyze("input.txt", 25), 228668);
}
