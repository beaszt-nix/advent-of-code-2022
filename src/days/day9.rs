use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    ops::{Add, Sub},
};

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct GridPoint {
    x: i32,
    y: i32,
}

impl GridPoint {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn move_x(&self, steps: i32) -> Self {
        return Self {
            y: self.y,
            x: self.x + steps,
        };
    }

    pub fn move_y(&self, steps: i32) -> Self {
        return Self {
            x: self.x,
            y: self.y + steps,
        };
    }
}

impl Add for GridPoint {
    type Output = GridPoint;

    fn add(self, other: Self) -> Self::Output {
        return Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Sub for GridPoint {
    type Output = GridPoint;

    fn sub(self, other: Self) -> Self::Output {
        return Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

fn move_tail(head: &GridPoint, tail: &GridPoint) -> GridPoint {
    let dist = *head - *tail;
    let (dx, dy) = (dist.x.signum(), dist.y.signum());

    if dist.x.abs() == 2 || dist.y.abs() == 2 {
        tail.move_x(dx).move_y(dy)
    } else {
        tail.clone()
    }
}

fn move_rope(
    rope: Vec<GridPoint>,
    head_move: GridPoint,
    visited: &mut HashSet<GridPoint>,
) -> Vec<GridPoint> {
    let head = rope[0] + head_move;
    let mut new_rope = vec![head];

    let mut last = head;
    for tail in rope.iter().skip(1) {
        let new_position = move_tail(&last, tail);
        new_rope.push(new_position);
        last = new_position;
    }

    visited.insert(last);
    return new_rope;
}

pub fn run(num_knots: usize) -> usize {
    let file = File::open("inputs/day9.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let rope = vec![GridPoint::new(0, 0); num_knots];
    let mut visited: HashSet<GridPoint> = HashSet::new();

    let res = reader
        .lines()
        .map(|line| line.unwrap().to_owned())
        .map(|line| {
            let token: Vec<&str> = line.split(" ").collect();

            match token[..] {
                ["R", steps] => vec![GridPoint::new(1, 0); steps.parse().unwrap()],
                ["L", steps] => vec![GridPoint::new(-1, 0); steps.parse().unwrap()],
                ["U", steps] => vec![GridPoint::new(0, 1); steps.parse().unwrap()],
                ["D", steps] => vec![GridPoint::new(0, -1); steps.parse().unwrap()],
                _ => panic!("Unexpected input"),
            }
        })
        .flatten()
        .fold(rope, |current_rope, head_move| {
            move_rope(current_rope, head_move, &mut visited)
        });

    return visited.len();
}
