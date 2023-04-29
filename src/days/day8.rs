use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn get_visible_trees(grid: &Vec<Vec<i8>>) -> HashSet<(usize, usize)> {
    let n = grid.len();
    let m = grid[0].len();

    let mut visible_trees = HashSet::new();

    let mut tallest_from_up = vec![-1; m];
    let mut tallest_from_down = vec![-1; m];
    for i in 0..n {
        let mut tallest_from_left = -1;
        let mut tallest_from_right = -1;
        for j in 0..m {
            if grid[i][j] > tallest_from_left {
                tallest_from_left = i8::max(tallest_from_left, grid[i][j]);
                visible_trees.insert((i, j));
            }
            if grid[i][m - 1 - j] > tallest_from_right {
                tallest_from_right = i8::max(tallest_from_right, grid[i][m - 1 - j]);
                visible_trees.insert((i, m - 1 - j));
            }
            if grid[i][j] > tallest_from_up[j] {
                tallest_from_up[j] = i8::max(tallest_from_up[j], grid[i][j]);
                visible_trees.insert((i, j));
            }
            if grid[n - 1 - i][j] > tallest_from_down[j] {
                tallest_from_down[j] = i8::max(tallest_from_down[j], grid[n - 1 - i][j]);
                visible_trees.insert((n - 1 - i, j));
            }
        }
    }

    return visible_trees;
}

fn get_scenic_score((i, j): (usize, usize), grid: &Vec<Vec<i8>>) -> usize {
    let mut left_score = 0;
    let mut right_score = 0;
    let mut up_score = 0;
    let mut down_score = 0;

    for col_iterator in j + 1..grid[i].len() {
        right_score += 1;
        if grid[i][j] <= grid[i][col_iterator] {
            break;
        }
    }

    for col_iterator in (0..j).rev() {
        left_score += 1;
        if grid[i][j] <= grid[i][col_iterator] {
            break;
        }
    }

    for row_iterator in i + 1..grid.len() {
        down_score += 1;
        if grid[i][j] <= grid[row_iterator][j] {
            break;
        }
    }

    for row_iterator in (0..i).rev() {
        up_score += 1;
        if grid[i][j] <= grid[row_iterator][j] {
            break;
        }
    }

    return left_score * right_score * up_score * down_score;
}

pub fn part1() -> usize {
    let file = File::open("inputs/day8.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let grid: Vec<Vec<i8>> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .to_string()
                .bytes()
                .map(|char| (char - b'0') as i8)
                .collect()
        })
        .collect();

    return get_visible_trees(&grid).len();
}

pub fn part2() -> usize {
    let file = File::open("inputs/day8.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let grid: Vec<Vec<i8>> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .to_string()
                .bytes()
                .map(|char| (char - b'0') as i8)
                .collect()
        })
        .collect();

    return get_visible_trees(&grid)
        .into_iter()
        .filter(|(i, j)| (*i != 0) && (*j != 0))
        .map(|idx| get_scenic_score(idx, &grid))
        .max()
        .unwrap();
}
