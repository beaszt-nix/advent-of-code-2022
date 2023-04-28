use std::{
    cell::RefCell,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    rc::Rc,
};

use super::types::{AstNode, FsTreeNode, FsTreeNodeRef, Path};

pub fn parse_input() -> FsTreeNodeRef {
    let file = File::open("inputs/day7.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let commands = reader
        .lines()
        .map(|line| line.unwrap().parse::<AstNode>().unwrap());

    let root = Rc::new(RefCell::new(FsTreeNode {
        parent: Option::None,
        size: 0,
        children: HashMap::new(),
    }));

    let mut current = root.clone();
    for command in commands {
        match command {
            AstNode::Ls => continue,
            AstNode::Cd(path) => {
                current = change_current_directory(&current, &root, path);
            }
            AstNode::Dir(dir_name) => {
                create_child(&current, dir_name, 0);
            }
            AstNode::File(file_name, size) => {
                create_child(&current, file_name, size);
            }
        }
    }
    root
}

pub fn change_current_directory(
    current: &FsTreeNodeRef,
    root: &FsTreeNodeRef,
    path: Path,
) -> FsTreeNodeRef {
    match path {
        Path::Root => Rc::clone(root),
        Path::Parent => Rc::clone(current.borrow().parent.as_ref().unwrap()),
        Path::Subdir(dir_name) => Rc::clone(current.borrow().children.get(&dir_name).unwrap()),
    }
}

pub fn create_child(node: &FsTreeNodeRef, name: String, size: usize) {
    let children = &mut node.as_ref().borrow_mut().children;
    children.insert(
        name,
        Rc::new(RefCell::new(FsTreeNode {
            parent: Some(Rc::clone(&node)),
            size,
            children: HashMap::new(),
        })),
    );
}

