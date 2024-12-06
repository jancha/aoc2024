use std::process::Command;

fn main() {
    let mut total = 0;
    let times = 100;
    let puzzles = 12;
    for i in 1..=puzzles {
        total += bench(
            times,
            &format!("../puzzle{i}"),
            &format!("target/release/puzzle{i}"),
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
