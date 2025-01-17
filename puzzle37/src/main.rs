use std::fs;
fn main() {
    println!("{}", analyze("input.txt"));
}

fn analyze(file: &str) -> usize {
    let file = fs::read_to_string(file).expect("Could not read file?");
    let data: Vec<&str> = file.trim().split("\n").map(|x| x.trim()).collect();
    let iter = data.iter();

    let mut colors: Vec<&str> = vec![];

    let mut colors_set = false;

    let mut found = 0;
    for i in iter {
        if !colors_set {
            colors = i.trim().split(", ").collect();
            compact_colors(&mut colors);
            colors_set = true;
        } else if i.is_empty() {
            continue;
        } else if sequence_possible(i.trim(), &colors, None) {
            found += 1;
        }
    }
    found
}

fn compact_colors(colors: &mut Vec<&str>) {
    let mut unique_colors: Vec<&str> = vec![];

    for color in colors.iter() {
        if !sequence_possible(color, colors, Some(*color)) {
            unique_colors.push(*color);
        }
    }
    *colors = unique_colors.to_vec();
}
fn sequence_possible(sequence: &str, colors: &Vec<&str>, skip: Option<&str>) -> bool {
    if sequence.is_empty() {
        return true;
    }
    for i in colors {
        if let Some(skip) = skip {
            if skip == *i {
                continue;
            }
        }
        if sequence.len() < (*i).len() {
            continue;
        }
        if sequence[0..(*i).len()] == **i
            && sequence_possible(&sequence[i.len()..sequence.len()], colors, skip)
        {
            return true;
        }
    }
    false
}

#[test]
fn test_1() {
    assert_eq!(analyze("test.txt"), 6);
}
