use std::{fs::File, io::{BufReader, BufRead}};

pub fn run(file_name: &str) -> i64 {
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
