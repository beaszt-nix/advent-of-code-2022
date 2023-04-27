use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn parse(line: String) -> Option<[(usize, usize); 2]> {
    let (left, right) = line.split_once(",")?;
    let (a, b) = left.split_once("-")?;
    let (c, d) = right.split_once("-")?;
    return Some([
        (a.parse::<usize>().ok()?, b.parse::<usize>().ok()?),
        (c.parse::<usize>().ok()?, d.parse::<usize>().ok()?),
    ]);
}

fn contains((a, b): &(usize, usize), (c, d): &(usize, usize)) -> bool {
    return (a..=b).contains(&c) && (a..=b).contains(&d);
}

// Problem guarantees that a < b and c < d
fn overlaps((a, b): &(usize, usize), (c, d): &(usize, usize)) -> bool {
    return (a..=b).contains(&c) || (c..=d).contains(&b);
}

pub fn part1() -> usize {
    let file = File::open("inputs/day4.txt").expect("Unable to open file");
    let reader = BufReader::new(file);
    return reader
        .lines()
        .filter_map(|line| parse(line.unwrap()))
        .filter(|[a, b]| contains(&a, &b) || contains(&b, &a))
        .count();
}

pub fn part2() -> usize {
    let file = File::open("inputs/day4.txt").expect("Unable to open file");
    let reader = BufReader::new(file);
    return reader
        .lines()
        .filter_map(|line| parse(line.unwrap()))
        .filter(|[a, b]| overlaps(&a, &b) || overlaps(&b, &a))
        .count();
}
