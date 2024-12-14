use std::env;
use std::process::Command;

fn main() {
    let args: Vec<_> = env::args().collect();

    let mut total = 0;
    let times = 100;
    let mut start = 1;
    let mut end = 22;
    if args.len() > 1 {
        start = args.get(1).unwrap().parse().unwrap();
    }
    if args.len() > 2 {
        end = args.get(2).unwrap().parse().unwrap();
    }
    for i in start..=end {
        total += bench(
            times,
            &format!("../puzzle{i}"),
            &format!("./target/release/puzzle{i}"),
        );
    }
    println!("Total: {}ms", total as f64 / times as f64);
}

fn bench(times: usize, dir: &str, command: &str) -> i64 {
    let time_start = chrono::Utc::now();

    for _i in 0..times {
        Command::new(command)
            .current_dir(dir)
            .output()
            .expect("failed to execute process");
    }
    let time_end = chrono::Utc::now();

    let ms = (time_end - time_start).num_milliseconds();

    println!(
        "Executed {}, {} times, took: {} ms, average: {} ms, {} s",
        command,
        times,
        ms,
        ms as f64 / times as f64,
        ms as f64 / times as f64 / 1000.
    );
    ms
}
