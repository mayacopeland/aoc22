use std::{fs::read_to_string};
use crate::downloader;

pub fn day_all(token: String) {
    day_one(token);
}

pub fn day_one(token: String) {
    println!("Day One: ");
    downloader::get_task(1, token);
    
    let file: String = read_to_string("src/task_input/task1.txt").unwrap();
    let elves: Vec<i32> = file.split("\n\n").map(|xs| {
        let elf: Vec<i32> = xs.split("\n").map(|x| x.parse::<i32>().unwrap_or(0)).collect(); 
        let sum: i32 = elf.iter().sum();
        sum
    }).collect();
    let mut cloned: Vec<i32> = elves.clone();
    let max: &i32 = elves.iter().max().unwrap();

    println!("Part One Solution: {}", max);
    
    cloned.sort();
    let sum: i32 = cloned.iter().rev().take(3).sum();
    
    println!("Part Two Solution: {}", sum);
    println!("==============");
}