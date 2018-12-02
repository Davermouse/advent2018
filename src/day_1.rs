use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

pub fn run() {
    println!("Day 1!");

    let mut f = File::open("day_1.txt").expect("File not found");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("Unable to load file");

    let nums = s.split_whitespace().map(
        |string_num| { string_num.parse::<i32>().expect("Unable to parse int") }).collect::<Vec<_>>();

    let mut frequency = 0;
    for num in nums.iter() {
        frequency += num;
    }

    println!("Final frequency: {}", frequency);

    let mut seen_freq : HashSet<i32> = HashSet::new();
    frequency = 0;
    'outer: loop {
        for num in nums.iter() {
            frequency += num;

            if seen_freq.contains(&frequency) {
                println!("Repeated frequency: {}", frequency);
                break 'outer;
            }

            seen_freq.insert(frequency);
        }
    }
}
