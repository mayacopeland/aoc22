use std::{fs::read_to_string};
use crate::downloader;

pub fn day_all(token: String) {
    day_one(token.clone());
    day_two(token.clone());
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

pub fn day_two(token: String) {
    println!("Day Two: ");
    downloader::get_task(2, token);

    // X = Rock, Y = Paper, Z is Scissors
    // A = Rock, B = Paper, C is Scissors
    // 1         2          3

    // please clean this up later
    let binding = read_to_string("src/task_input/task2.txt").unwrap();
    let rounds = binding.split("\n");
    let rounds_played: Vec<i32> = rounds.clone().map(|xs| {
        let round: Vec<&str>= xs.split(" ").collect();
        let mut points = 0;
        match round[0] {
            "A" => {
                match round[1] {
                    "X" => points += 3 + 1, // draw
                    "Y" => points += 6 + 2, // win
                    "Z" => points += 0 + 3, // lose
                    _ => points += 0
                }
            },
            "B" => {
                match round[1] {
                    "X" => points += 0 + 1, // lose
                    "Y" => points += 3 + 2, // draw
                    "Z" => points += 6 + 3, // win
                    _ => points += 0
                }
            },
            "C" => {
                match round[1] {
                    "X" => points += 6 + 1, // win
                    "Y" => points += 0 + 2, // lose
                    "Z" => points += 3 + 3, // draw
                    _ => points += 0
                }
            },
            _ => points += 0
        }
        points
    }).collect();
    let part_one: i32 = rounds_played.iter().sum();
    println!("Part One Solution: {}", part_one);
    // ================================ \\
    let rounds_outcome: Vec<i32> = rounds.clone().map(|xs| {
        let mut points = 0;
        let round: Vec<&str>= xs.split(" ").collect();        
        // add points for winning etc
        match round[1] {
            "X" => {
                points += 0;
                match round[0] {
                    "A" => points += 3,
                    "B" => points += 1,
                    "C" => points += 2,
                    _ => points += 0
                }; 
            },
            "Y" => {
                points += 3;
                match round[0] {
                    "A" => points += 1,
                    "B" => points += 2,
                    "C" => points += 3,
                    _ => points += 0
                };
            },
            "Z" => {
                points += 6;
                match round[0] {
                    "A" => points += 2,
                    "B" => points += 3,
                    "C" => points += 1,
                    _ => points += 0
                };
            },
            _ => points += 0
        }
      
        points
    }).collect();
    let part_two: i32 = rounds_outcome.iter().sum();

    println!("Part two Solution: {}", part_two);
    println!("==============");
}