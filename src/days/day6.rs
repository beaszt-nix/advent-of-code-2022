use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, Read},
};

fn check_distinct(input: &[u8]) -> bool {
    let mut set = HashSet::new();
    input.to_owned().into_iter().all(move |x| set.insert(x))
}

pub fn part1() -> usize {
    let file = File::open("inputs/day6.txt").expect("Unable to open file");
    let mut reader = BufReader::new(file);

    let mut message = Vec::new();
    reader.read_to_end(&mut message);

    let (idx, _) = message
        .windows(4)
        .enumerate()
        .skip_while(|(_, window)| !check_distinct(window))
        .next()
        .unwrap();
    return idx + 4;
}

pub fn part2() -> usize {
    let file = File::open("inputs/day6.txt").expect("Unable to open file");
    let mut reader = BufReader::new(file);

    let mut message = Vec::new();
    reader.read_to_end(&mut message);

    let (idx, _) = message
        .windows(14)
        .enumerate()
        .skip_while(|(_, window)| !check_distinct(window))
        .next()
        .unwrap();
    return idx + 14;
}
