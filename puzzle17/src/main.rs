use std::fs;

fn main() {
    println!("{}", analyze("input.txt"));
}

#[derive(Debug)]
struct DiskEntry {
    block: Option<usize>,
}

impl DiskEntry {
    fn new(block: Option<usize>) -> DiskEntry {
        DiskEntry { block }
    }
}

fn analyze(file: &str) -> usize {
    let file: String = fs::read_to_string(file).expect("Could not read file?");
    let file = file.trim();

    let mut disk: Vec<DiskEntry> = vec![];

    for (indice, size) in file.char_indices() {
        let size: u8 = size.to_digit(10).unwrap() as u8;
        let id = indice / 2;
        if indice % 2 == 0 {
            for _i in 0..size {
                disk.push(DiskEntry::new(Some(id)));
            }
        } else {
            for _i in 0..size {
                disk.push(DiskEntry::new(None));
            }
        }
    }

    let disk_len = disk.len() as isize;
    let mut last_check = disk_len;

    for i in 0..disk_len {
        let entry = disk.get(i as usize).unwrap();
        if entry.block.is_none() {
            for j in 1..last_check - i - 1 {
                let index = last_check - j;
                let cmp = disk.get(index as usize).unwrap();
                if cmp.block.is_some() {
                    disk.swap(i as usize, index as usize);
                    last_check = index;
                    break;
                }
            }
        }
    }

    let mut sum = 0;
    disk.iter().enumerate().for_each(|(index, entry)| {
        if let Some(block) = &entry.block {
            sum += index * block;
        }
    });
    sum
}
#[test]
fn test_1() {
    assert_eq!(analyze("test.txt"), 1928);
    assert_eq!(analyze("input.txt"), 6211348208140);
}
