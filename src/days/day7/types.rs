use std::{str::FromStr, rc::Rc, cell::RefCell, collections::HashMap};

pub enum Path {
    Root,
    Parent,
    Subdir(String),
}

pub enum AstNode {
    Cd(Path),
    Ls,
    File(String, usize),
    Dir(String),
}

#[derive(Debug)]
pub struct InputParseError;

impl FromStr for AstNode {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let token: Vec<&str> = s.split(" ").collect();
        match token[..] {
            ["$", "cd", "/"] => Ok(AstNode::Cd(Path::Root)),
            ["$", "cd", ".."] => Ok(AstNode::Cd(Path::Parent)),
            ["$", "cd", path] => Ok(AstNode::Cd(Path::Subdir(path.to_string()))),
            ["$", "ls"] => Ok(AstNode::Ls),
            ["dir", dir_name] => Ok(AstNode::Dir(dir_name.to_string())),
            [file_size, file_name] => Ok(AstNode::File(
                file_name.to_string(),
                file_size.parse::<usize>().map_err(|_| InputParseError)?,
            )),
            _ => Err(InputParseError),
        }
    }
}

pub type FsTreeNodeRef = Rc<RefCell<FsTreeNode>>;

pub struct FsTreeNode {
    pub parent: Option<FsTreeNodeRef>,
    pub size: usize,
    pub children: HashMap<String, FsTreeNodeRef>,
}
