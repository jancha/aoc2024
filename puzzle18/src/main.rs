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
    let mut files: Vec<(usize, usize, usize)> = vec![];
    let mut empty_blocks: Vec<(usize, usize)> = vec![];
    let mut pos = 0;

    for (indice, size) in file.char_indices() {
        let size: usize = size.to_digit(10).unwrap() as usize;
        let id = indice / 2;
        if indice % 2 == 0 {
            files.push((pos, id, size));
            for _i in 0..size {
                disk.push(DiskEntry::new(Some(id)));
            }
        } else {
            empty_blocks.push((pos, size));
            for _i in 0..size {
                disk.push(DiskEntry::new(None));
            }
        }
        pos += size;
    }

    let files_len = files.len();
    for i in 0..files_len {
        let file_pos = files_len - i - 1; // start from largest
        let file = files.get(file_pos).unwrap();

        let slot = empty_blocks
            .iter()
            .position(|(start, size)| *size >= file.2 && *start < file.0);

        if let Some(slot) = slot {
            let empty_block = empty_blocks.get_mut(slot).unwrap();

            for j in 0..file.2 {
                disk.swap(empty_block.0 + j, file.0 + j);
            }

            let remaining_free = empty_block.1 - file.2;
            if remaining_free > 0 {
                empty_block.0 += empty_block.1 - remaining_free;
                empty_block.1 = remaining_free;
            } else {
                empty_blocks.remove(slot);
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
    assert_eq!(analyze("test.txt"), 2858);
    assert_eq!(analyze("input.txt"), 6239783302560);
}
