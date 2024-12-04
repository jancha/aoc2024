use std::fs;

const WORD_LEN: usize = 4;
const CHUNK_LEN: usize = 3;

fn main() {
    println!("{}", get_words("input.txt"));
}

fn get_words(file: &str) -> usize {
    let file = fs::read_to_string(file).expect("Could not read file?");
    let lines: Vec<&str> = file.split("\n").collect();
    let line_len = lines.first().unwrap().len();
    let mut count = 0;
    for (line_num, line) in lines.iter().enumerate() {
        for (char_pos, c) in line.char_indices() {
            if c == 'X' {
                count += find_word(&lines, line, line_len, line_num, char_pos);
            }
        }
    }
    count
}

fn find_word(
    lines: &Vec<&str>,
    line: &str,
    line_len: usize,
    line_num: usize,
    char_pos: usize,
) -> usize {
    let mut found = 0;
    let line_count = lines.len();
    let mut word_found = 2;
    if char_pos >= CHUNK_LEN {
        let word = &line[char_pos - CHUNK_LEN..=char_pos];
        if word == "SAMX" {
            found += 1;
        }
        // check up left
        if line_num >= CHUNK_LEN {
            let y0 = line_num - CHUNK_LEN;
            let y1 = line_num - 1;
            for y in y0..=y1 {
                let x = line_num - y;
                let c1 = get_word_char(x);
                let c2 = get_char_at(lines, char_pos - x, y);

                if c1 != c2 {
                    word_found -= 1;
                    break;
                }
            }
        } else {
            word_found -= 1;
        }
        if line_num < line_count - WORD_LEN {
            let y0 = line_num + 1;
            let y1 = line_num + CHUNK_LEN;
            for y in y0..=y1 {
                let x = y - line_num;

                let c1 = get_word_char(x);
                let c2 = get_char_at(lines, char_pos - x, y);

                if c1 != c2 {
                    word_found -= 1;
                    break;
                }
            }
        } else {
            word_found -= 1;
        }
        found += word_found;
    }
    // check right
    if char_pos < line_len - CHUNK_LEN {
        let word = &line[char_pos..=CHUNK_LEN + char_pos];
        if word == "XMAS" {
            found += 1;
        }
        // check up right
        word_found = 2;
        if line_num >= CHUNK_LEN {
            let y0 = line_num - CHUNK_LEN;
            let y1 = line_num - 1;
            for y in y0..=y1 {
                let x = line_num - y;
                if get_word_char(x) != get_char_at(lines, char_pos + x, y) {
                    word_found -= 1;
                    break;
                }
            }
        } else {
            word_found -= 1;
        }

        if line_num < line_count - WORD_LEN {
            let y0 = line_num + 1;
            let y1 = line_num + CHUNK_LEN;
            for y in y0..=y1 {
                let x = y - line_num;
                if get_word_char(x) != get_char_at(lines, char_pos + x, y) {
                    word_found -= 1;
                    break;
                }
            }
        } else {
            word_found -= 1;
        }
        found += word_found;
    }
    word_found = 2;
    // check up
    if line_num >= CHUNK_LEN {
        let y0 = line_num - CHUNK_LEN;
        let y1 = line_num - 1;
        for y in y0..=y1 {
            let x = line_num - y;
            if get_word_char(x) != get_char_at(lines, char_pos, y) {
                word_found -= 1;
                break;
            }
        }
    } else {
        word_found -= 1;
    }
    // check down
    if line_num < line_count - WORD_LEN {
        let y0 = line_num + 1;
        let y1 = line_num + CHUNK_LEN;
        for y in y0..=y1 {
            let x = y - line_num;
            if get_word_char(x) != get_char_at(lines, char_pos, y) {
                word_found -= 1;
                break;
            }
        }
    } else {
        word_found -= 1;
    }
    found += word_found;
    found
}

fn get_char_at(lines: &[&str], x: usize, y: usize) -> char {
    lines.get(y).unwrap().chars().nth(x).unwrap()
}

const WORD: &str = "XMAS";

fn get_word_char(x: usize) -> char {
    WORD.chars().nth(x).unwrap()
}

#[test]

fn test_1() {
    assert_eq!(get_words("test1.txt"), 8);
    assert_eq!(get_words("test2.txt"), 18);
    assert_eq!(get_words("input.txt"), 2427);
}
