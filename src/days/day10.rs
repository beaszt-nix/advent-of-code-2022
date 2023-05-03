use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(Debug)]
pub enum Operation {
    Noop,
    AddX(i32),
}

#[derive(Debug)]
pub struct Cpu {
    register_x: i32,
    cycle: usize,
    instructions: VecDeque<Operation>,
    deque_at: usize,
    buffer: [[char; 40]; 6],
}

impl Cpu {
    pub fn new(instructions: VecDeque<Operation>) -> Self {
        return Self {
            register_x: 1,
            cycle: 1,
            instructions,
            deque_at: 0,
            buffer: [[' '; 40]; 6],
        };
    }

    pub fn tick(&mut self) {
        match self.instructions.front() {
            Some(Operation::Noop) => {
                self.instructions.pop_front();
            }
            Some(Operation::AddX(increment)) => match self.cycle.cmp(&self.deque_at) {
                std::cmp::Ordering::Less => {}
                std::cmp::Ordering::Equal => {
                    self.register_x += increment;
                    self.instructions.pop_front();
                }
                std::cmp::Ordering::Greater => {
                    self.deque_at = self.cycle + 1;
                }
            },
            None => {}
        }
        self.cycle += 1;
    }

    pub fn part1(&mut self) -> i32 {
        let mut sum = 0;
        while self.cycle <= 220 {
            if self.cycle % 40 == 20 {
                sum += (self.cycle as i32) * self.register_x;
            }
            self.tick();
        }
        return sum;
    }

    pub fn fill_buffer(&mut self) {
        let width = 40;
        let cols = 6;
        let sprite_size = 3;

        while self.cycle <= width * cols {
            let pixel_x = (self.cycle - 1) % width;
            let pixel_y = (self.cycle - 1) / width;

            let is_pixel_covered =
                (self.register_x..(self.register_x + sprite_size)).contains(&(pixel_x as i32));

            self.buffer[pixel_y][pixel_x] = if is_pixel_covered { '█' } else { ' ' };
            self.tick();
        }
    }

    pub fn print_buffer(&self) {
        self.buffer
            .iter()
            .for_each(|line| println!("{}", line.iter().collect::<String>()));
    }

    pub fn part2(&mut self) {
        self.fill_buffer();
        self.print_buffer();
    }
}

impl FromStr for Operation {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split(" ").collect();
        match tokens[..] {
            ["noop"] => Ok(Operation::Noop),
            ["addx", num] => Ok(Operation::AddX(num.parse().map_err(|_| num.to_owned())?)),
            _ => Err(String::from("Unable to parse input")),
        }
    }
}

pub fn part1() -> i32 {
    let file = File::open("inputs/day10.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let operations = reader
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    let mut cpu = Cpu::new(operations);

    return cpu.part1();
}

pub fn part2() {
    let file = File::open("inputs/day10.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let operations = reader
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    let mut cpu = Cpu::new(operations);

    cpu.part2()
}
