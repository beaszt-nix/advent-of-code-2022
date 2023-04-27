use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Move {
    ROCK = 0,
    PAPER = 1,
    SCISSOR = 2,
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Move::ROCK, Move::SCISSOR) => Some(Ordering::Greater),
            (Move::SCISSOR, Move::ROCK) => Some(Ordering::Less),
            (x, y) => (*x as u8).partial_cmp(&(*y as u8)),
        }
    }
}

#[derive(Debug)]
struct MoveParseError;

impl FromStr for Move {
    type Err = MoveParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if ["A"].contains(&s) {
            Ok(Move::ROCK)
        } else if ["B"].contains(&s) {
            Ok(Move::PAPER)
        } else if ["C"].contains(&s) {
            Ok(Move::SCISSOR)
        } else {
            Err(MoveParseError)
        }
    }
}

impl Move {
    fn get_score(&self, other: &Self) -> i32 {
        1 + (*self as i32)
            + match self.partial_cmp(other) {
                Some(Ordering::Less) => 0,
                Some(Ordering::Equal) => 3,
                Some(Ordering::Greater) => 6,
                None => panic!("Undefined behaviour"),
            }
    }
}

fn part1_round_score(line: String) -> i32 {
    let (elf_move_token, your_move_token) = line.trim().split_once(" ").unwrap();
    let elf_move = elf_move_token.parse().unwrap();
    let your_move = match your_move_token {
        "X" => Move::ROCK,
        "Y" => Move::PAPER,
        "Z" => Move::SCISSOR,
        _ => panic!("Expected input only between [X,Y,Z]"),
    };
    return your_move.get_score(&elf_move);
}

fn part2_round_score(line: String) -> i32 {
    let (elf_move_token, your_move_token) = line.trim().split_once(" ").unwrap();
    let elf_move: Move = elf_move_token.parse().unwrap();
    match your_move_token {
        // % cannot be used, since % means remainder, not modulo.
        // solution depends on -1 % 3 == 3
        "X" => 1 + (((elf_move as i32) - 1).rem_euclid(3)) + 0,
        "Y" => 1 + (elf_move as i32).rem_euclid(3) + 3,
        "Z" => 1 + (((elf_move as i32) + 1).rem_euclid(3)) + 6,
        _ => panic!("Expected input only between [X,Y,Z]"),
    }
}


pub fn part1() -> i32 {
    let file = File::open("inputs/day2.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().trim().to_owned())
        .map(part1_round_score)
        .sum()
}

pub fn part2() -> i32 {
    let file = File::open("inputs/day2.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().trim().to_owned())
        .map(part2_round_score)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::days::day2::part1_round_score;
    use crate::days::day2::part2_round_score;

    #[test]
    fn part1_test_rock() {
        assert_eq!(part1_round_score("A X".to_string()), 4);
        assert_eq!(part1_round_score("A Y".to_string()), 8);
        assert_eq!(part1_round_score("A Z".to_string()), 3);
    }

    #[test]
    fn part1_test_paper() {
        assert_eq!(part1_round_score("B X".to_string()), 1);
        assert_eq!(part1_round_score("B Y".to_string()), 5);
        assert_eq!(part1_round_score("B Z".to_string()), 9);
    }

    #[test]
    fn part1_test_scissor() {
        assert_eq!(part1_round_score("C X".to_string()), 7);
        assert_eq!(part1_round_score("C Y".to_string()), 2);
        assert_eq!(part1_round_score("C Z".to_string()), 6);
    }

    #[test]
    fn part2_test_rock() {
        assert_eq!(part2_round_score("A X".to_string()), 3);
        assert_eq!(part2_round_score("A Y".to_string()), 4);
        assert_eq!(part2_round_score("A Z".to_string()), 8);
    }

    #[test]
    fn part2_test_paper() {
        assert_eq!(part2_round_score("B X".to_string()), 1);
        assert_eq!(part2_round_score("B Y".to_string()), 5);
        assert_eq!(part2_round_score("B Z".to_string()), 9);
    }

    #[test]
    fn part2_test_scissor() {
        assert_eq!(part2_round_score("C X".to_string()), 2);
        assert_eq!(part2_round_score("C Y".to_string()), 6);
        assert_eq!(part2_round_score("C Z".to_string()), 7);
    }
}
