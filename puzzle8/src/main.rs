use std::fs;

fn main() {
    println!("{}", get_words("input.txt"));
}

fn get_words(file: &str) -> usize {
    let file = fs::read_to_string(file).expect("Could not read file?");
    let lines: Vec<&str> = file.trim().split("\n").collect();
    let line_len = lines.first().unwrap().len();
    let mut count = 0;
    for (line_num, line) in lines.iter().enumerate() {
        for (char_pos, c) in line.char_indices() {
            if c == 'A' {
                count += find_cross(&lines, line_len, line_num, char_pos);
            }
        }
    }
    count
}

fn find_cross(lines: &[&str], line_len: usize, line_num: usize, char_pos: usize) -> usize {
    let line_count = lines.len();

    if line_num > 0 && line_num < line_count - 1 && char_pos > 0 && char_pos < line_len - 1 {
        let c1 = get_char_at(lines, char_pos - 1, line_num - 1);
        let c2 = get_char_at(lines, char_pos + 1, line_num + 1);

        if !(c1 == 'M' && c2 == 'S' || c1 == 'S' && c2 == 'M') {
            return 0;
        }
        let c3 = get_char_at(lines, char_pos + 1, line_num - 1);
        let c4 = get_char_at(lines, char_pos - 1, line_num + 1);
        if !(c3 == 'M' && c4 == 'S' || c3 == 'S' && c4 == 'M') {
            return 0;
        }
        return 1;
    }
    0
}

fn get_char_at(lines: &[&str], x: usize, y: usize) -> char {
    lines.get(y).unwrap().chars().nth(x).unwrap()
}

#[test]

fn test_1() {
    assert_eq!(get_words("test1.txt"), 4);
    assert_eq!(get_words("test2.txt"), 9);
    assert_eq!(get_words("input.txt"), 1900);
}
