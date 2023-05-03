mod days;

#[derive(Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

fn main() {
    // println!("Day1 Part1: {:?}", days::day1::part1());
    // println!("Day1 Part2: {:?}", days::day1::part2());
    // println!("Day2 Part1: {:?}", days::day2::part1());
    // println!("Day2 Part2: {:?}", days::day2::part2());
    // println!("Day3 Part1: {:?}", days::day3::part1());
    // println!("Day3 Part2: {:?}", days::day3::part2());
    // println!("Day4 Part1: {:?}", days::day4::part1());
    // println!("Day4 Part2: {:?}", days::day4::part2());
    // println!("Day5 Part1: {:?}", days::day5::part1());
    // println!("Day5 Part2: {:?}", days::day5::part2());
    // println!("Day6 Part1: {:?}", days::day6::part1());
    // println!("Day6 Part2: {:?}", days::day6::part2());
    // println!("Day7 Part1: {:?}", days::day7::part1());
    // println!("Day7 Part2: {:?}", days::day7::part2());
    // println!("Day8 Part1: {:?}", days::day8::part1());
    // println!("Day8 Part2: {:?}", days::day8::part2());
    // println!("Day9 Part1: {:?}", days::day9::run(2));
    // println!("Day9 Part2: {:?}", days::day9::run(10));
    // println!("Day10 Part1: {:?}", days::day10::part1());
    days::day10::part2();
    let mut prim_vec = vec![100; 3];
    let mut struct_vec = vec![Point::new(5, 5); 3];

    let mut index = 0;
    prim_vec[index] = 3;
    struct_vec[index] = Point::new(5,5);

}
