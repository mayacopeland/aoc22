use std::env;
mod days;
mod downloader;

fn main() {
    println!("Advent of Code 2022\n==============");
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 3);

    let binding = args.clone();
    let cookie = binding.get(1).unwrap();
    let day = binding.get(2).unwrap().parse::<i32>().unwrap();

    // Day one
    match day {
        1 => days::day_one(cookie.to_string()),
        _ => days::day_all(cookie.to_string())
    }
}
