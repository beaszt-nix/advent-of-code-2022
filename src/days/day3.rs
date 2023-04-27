use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn part1_logic(bag_items: String) -> i32 {
    let (first, second) = bag_items.split_at(bag_items.len() / 2);
    let first_set: HashSet<u8> = first.bytes().collect();
    let second_set: HashSet<u8> = second.bytes().collect();

    first_set
        .intersection(&second_set)
        .map(|item| match item {
            b'a'..=b'z' => (item - b'a') as i32 + 1,
            b'A'..=b'Z' => (item - b'A') as i32 + 27,
            _ => panic!("Expected only characters between a..z and A..Z"),
        })
        .sum()
}

pub fn part1() -> i32 {
    let file = File::open("inputs/day3.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().trim().to_owned())
        .map(part1_logic)
        .sum()
}

pub fn part2() -> i32 {
    let file = File::open("inputs/day3.txt").expect("Unable to open file");
    let reader = BufReader::new(file);
    let input: Vec<u8> = reader
        .lines()
        .map(|line| line.unwrap().trim().to_owned())
        .collect::<Vec<String>>()
        .chunks(3)
        .flat_map(|chunk| {
            chunk
                .iter()
                .map(|line| line.bytes().collect::<HashSet<u8>>())
                .reduce(|acc, set| acc.intersection(&set).cloned().collect())
                .unwrap()
                .into_iter()
        })
        .collect();

    return input
        .into_iter()
        .map(|item| match item {
            b'a'..=b'z' => (item - b'a') as i32 + 1,
            b'A'..=b'Z' => (item - b'A') as i32 + 27,
            _ => panic!("Expected only characters between a..z and A..Z"),
        })
        .sum();
}
