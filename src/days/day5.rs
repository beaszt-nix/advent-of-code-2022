use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(Clone, Copy, Debug)]
struct Crate {
    value: char,
}

#[derive(Clone, Copy, Debug)]
struct Move {
    pub num_crates: usize,
    pub source_stack: usize,
    pub target_stack: usize,
}

struct MoveParseError;

impl FromStr for Move {
    type Err = MoveParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split(" ").collect();
        match tokens[..] {
            ["move", num_crates, "from", source_stack, "to", target_stack] => Ok(Move {
                num_crates: num_crates.parse::<usize>().unwrap(),
                source_stack: source_stack.parse::<usize>().unwrap(),
                target_stack: target_stack.parse::<usize>().unwrap(),
            }),
            _ => Err(MoveParseError),
        }
    }
}

fn parse_crate(line: &String) -> Vec<Option<Crate>> {
    line.chars()
        .collect::<Vec<char>>()
        .chunks(4)
        .into_iter()
        .map(|token| {
            if token[0] == '[' && token[2] == ']' {
                Some(Crate { value: token[1] })
            } else {
                None
            }
        })
        .collect()
}

fn parse_input(lines: Vec<String>) -> (Vec<Vec<Crate>>, Vec<Move>) {
    let val: Vec<Vec<Option<Crate>>> = lines
        .iter()
        .take_while(|line| line.starts_with("["))
        .map(|line| parse_crate(line))
        .collect();

    let moves: Vec<Move> = lines
        .iter()
        .skip_while(|line| !line.starts_with("m"))
        .filter_map(|line| line.parse().ok())
        .collect();

    let mut crate_stack: Vec<Vec<Crate>> = vec![vec![]; val[0].len()];
    for row in val.iter().rev() {
        for (idx, item) in row.into_iter().enumerate() {
            if item.is_some() {
                crate_stack[idx].push(item.unwrap());
            }
        }
    }
    return (crate_stack, moves);
}

pub fn part1() -> String {
    let file = File::open("inputs/day5.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let (mut crate_stack, moves) = parse_input(
        reader
            .lines()
            .map(|line| line.unwrap().to_owned())
            .collect(),
    );

    for crate_move in moves.iter() {
        for _ in 0..crate_move.num_crates {
            let incoming_box = crate_stack[crate_move.source_stack - 1].pop().unwrap();
            crate_stack[crate_move.target_stack - 1].push(incoming_box);
        }
    }

    return crate_stack
        .iter()
        .filter_map(|stack| stack.iter().last())
        .map(|item| item.value)
        .collect();
}

pub fn part2() -> String {
    let file = File::open("inputs/day5.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let (mut crate_stack, moves) = parse_input(
        reader
            .lines()
            .map(|line| line.unwrap().to_owned())
            .collect(),
    );

    for crate_move in moves.iter() {
        let from_stack = &mut crate_stack[crate_move.source_stack - 1];
        let mut incoming_boxes = from_stack.split_off(from_stack.len() - crate_move.num_crates);
        crate_stack[crate_move.target_stack - 1].append(&mut incoming_boxes);
    }

    return crate_stack
        .iter()
        .filter_map(|stack| stack.iter().last())
        .map(|item| item.value)
        .collect();
}
