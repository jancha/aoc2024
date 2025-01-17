use std::collections::HashMap;
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
    let mut mutations: HashMap<String, usize> = HashMap::new();
    for i in iter {
        if !colors_set {
            colors = i.trim().split(", ").collect();

            colors.sort();

            colors.sort_by(|a, b| {
                let l1 = a.len();
                let l2 = b.len();
                l1.cmp(&l2)
            });
            colors.reverse();

            colors_set = true;
        } else if i.is_empty() {
            continue;
        } else {
            let se_opt = sequence_possible(i.trim(), &colors, None, &mut mutations);
            found += se_opt;
        }
    }
    found
}

fn sequence_possible(
    sequence: &str,
    colors: &Vec<&str>,
    skip: Option<&str>,
    dict: &mut HashMap<String, usize>,
) -> usize {
    if sequence.is_empty() {
        return 1;
    }
    let mut routes = 0;
    for i in colors {
        if let Some(skip) = skip {
            if skip == *i {
                continue;
            }
        }
        if sequence.len() < (*i).len() {
            continue;
        }
        if sequence[0..(*i).len()] == **i {
            let remainder = &sequence[i.len()..sequence.len()];

            if let Some(sub_routes) = dict.get(remainder) {
                routes += sub_routes;
            } else {
                let sub_routes =
                    sequence_possible(&sequence[i.len()..sequence.len()], colors, skip, dict);
                dict.insert(remainder.to_string(), sub_routes);
                routes += sub_routes;
            }
        }
    }

    routes
}

#[test]
fn test_1() {
    assert_eq!(analyze("test.txt"), 16);
}
