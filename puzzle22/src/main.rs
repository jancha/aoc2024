use std::collections::HashMap;
use std::fs;

fn main() {
    println!("{}", analyze("input.txt", 75));
}

type Hash = HashMap<usize, HashMap<usize, usize>>;

fn analyze(file: &str, mutations: u8) -> usize {
    let file = fs::read_to_string(file).expect("Could not read file?");
    let map: Vec<&str> = file.trim().split(' ').collect();

    let mut hashes: Hash = HashMap::new();

    let stones: Vec<usize> = map
        .iter()
        .map(|x| {
            let y: usize = x.parse().unwrap();
            y
        })
        .collect();

    let mut stone_count = 0;

    for i in stones {
        stone_count += mutate_stone(i, mutations, &mut hashes);
    }
    stone_count
}

fn mutate_stone(stone: usize, mutations: u8, hashes: &mut Hash) -> usize {
    let stones = get_hash(stone, mutations, hashes);

    stones.values().sum()
}

fn learn_stone(stone: usize, mutations: u8, hashes: &mut Hash) {
    let mut stones: HashMap<usize, usize> = HashMap::new();
    stones.insert(stone, 1);

    let push_stone = |stone: usize, count: usize, stones: &mut HashMap<usize, usize>| {
        if let Some(entry) = stones.get_mut(&stone) {
            *entry += count;
        } else {
            stones.insert(stone, count);
        }
    };
    for _i in 0..mutations {
        let mut new_stones = HashMap::new();

        for (stone, _count) in stones {
            match stone {
                0 => push_stone(1, _count, &mut new_stones),
                x if ((x as usize).ilog(10_usize) + 1) % 2 == 0 => {
                    let mid_dividor: usize = 10_usize.pow(((x as usize).ilog(10_usize) + 1) / 2);
                    let stone1 = x / mid_dividor;
                    let stone2 = x - stone1 * mid_dividor;
                    push_stone(stone1, _count, &mut new_stones);
                    push_stone(stone2, _count, &mut new_stones);
                }
                x => push_stone(x * 2024, _count, &mut new_stones),
            };
        }
        stones = new_stones;
    }

    hashes.insert(stone, stones);
}

fn get_hash(stone: usize, mutations: u8, hashes: &mut Hash) -> HashMap<usize, usize> {
    if let Some(hash) = hashes.get(&stone) {
        return hash.clone();
    }
    learn_stone(stone, mutations, hashes);

    hashes.get(&stone).unwrap().clone()
}

#[test]
fn test_1() {
    assert_eq!(analyze("test.txt", 25), 55312);
    assert_eq!(analyze("input.txt", 25), 228668);
    assert_eq!(analyze("input.txt", 75), 270673834779359);
}
