use self::{io::parse_input, types::FsTreeNodeRef};

mod io;
mod types;

pub fn get_dir_sizes(root: &FsTreeNodeRef, dir_sizes: &mut Vec<usize>) -> usize {
    let val = root.borrow();
    if val.size != 0 {
        return val.size;
    } else {
        let sum = val
            .children
            .iter()
            .map(|(_, v)| get_dir_sizes(v, dir_sizes))
            .sum();
        dir_sizes.push(sum);
        sum
    }
}

pub fn part1() -> usize {
    let root = parse_input();
    let mut dir_sizes = Vec::new();
    get_dir_sizes(&root, &mut dir_sizes);

    return dir_sizes.into_iter().filter(|size| *size < 100000).sum();
}

pub fn part2() -> usize {
    let root = parse_input();
    let mut dir_sizes = Vec::new();
    let remaining_size = 70000000 - get_dir_sizes(&root, &mut dir_sizes);
    let size_to_free = 30000000 - remaining_size;

    dir_sizes.sort();
    return upper_bound(&dir_sizes, size_to_free).unwrap();
}

fn upper_bound(vec: &Vec<usize>, target: usize) -> Option<usize> {
    let mut l = 0;
    let mut h = vec.len() - 1;

    let mut res = None;
    while l <= h {
        let mid = (l + h) / 2;
        if vec[mid] >= target {
            res = Some(vec[mid]);
            h = mid - 1;
        } else {
            l = mid + 1;
        }
    }
    res
}
