use std::{fs::File, io::{BufReader, BufRead}};

pub fn part1(file_name: &str) -> i64 {
    let file = File::open(file_name).expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut sum = 0;
    let mut max_calories = i64::MIN;
    for line in reader.lines() {
        if let Ok(val) = line.unwrap().trim().parse::<i64>() {
            sum += val;
        } else {
            max_calories = max_calories.max(sum);
            sum = 0;
        }
    }
    return max_calories;
}
pub fn part2(file_name: &str) -> i64 {
    let file = File::open(file_name).expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut elf_calories = vec![];
    let mut sum = 0;
    for line in reader.lines() {
        if let Ok(val) = line.unwrap().trim().parse::<i64>() {
            sum += val;
        } else {
            elf_calories.push(sum);
            sum = 0;
        }
    }
    elf_calories.sort_by(|a, b| b.cmp(a));
    if elf_calories.len() < 3 {
        panic!("Size less than 3");
    } else {
        elf_calories[0..3].iter().sum()
    }
}
