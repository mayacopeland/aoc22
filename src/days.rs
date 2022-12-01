use std::{fs::read_to_string};
use crate::downloader;

pub fn day_all(token: String) {
    day_one(token);
}

pub fn day_one(token: String) {
    println!("Day One: ");
    downloader::get_task(1, token);
    let file = read_to_string("src/task_input/task1.txt").unwrap();
    let file_split: Vec<&str> = file.split("\n\n").collect();
    let mut sum_vector: Vec<i32> = vec!(); 
    
    // TODO: improve hacky code
    for str in file_split {
        let elf: Vec<&str> = str.split("\n").collect();
        let mut sum = 0;
        for calorie in elf {
            if calorie != "" {
                let calorie_int = calorie.parse::<i32>().unwrap();
                sum += calorie_int;
            }
        }
        sum_vector.push(sum);
    }
    let mut cloned = sum_vector.clone();
    let max = sum_vector.iter().max().unwrap();

    println!("Part One Solution: {}", max);
    
    cloned.sort();
    let sum: i32 = cloned.iter().rev().take(3).sum();
    
    println!("Part Two Solution: {}", sum);
    println!("==============");
}