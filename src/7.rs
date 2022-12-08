use std::{cell::RefCell, rc::Rc};

mod utils;

type VS = Vec<String>;
type VVS = Vec<VS>;
type VVVS = Vec<VVS>;
type RRIN = Rc<RefCell<INode>>;

#[derive(Debug)]
struct INode {
    name: String,
    size: usize,
    is_dir: bool,
    children: Vec<RRIN>,
    parent: Option<RRIN>,
}

impl INode {
    pub fn new(name: String, size: usize, is_dir: bool, parent: Option<RRIN>) -> Self {
        Self {
            name,
            size,
            is_dir,
            children: vec![],
            parent,
        }
    }
}

#[derive(Debug)]
struct Directory {
    cmds: VVS,
    ls_list: VVVS,
    root: Option<RRIN>,
    dir_sizes: Vec<usize>,
    max_size: usize,
    least_size_needed_to_delete: usize,
    cur_least_dir: Option<RRIN>,
    cur_least: usize,
}

impl Directory {
    pub fn new() -> Self {
        let data = utils::fast_get_file_data_as_vec();

        let mut cmds: VVS = vec![];
        let mut ls_list: VVVS = vec![];
        let mut cur_ls_list: VVS = vec![];
        let mut is_cmd = true;
        let mut i: usize = 0;
        while i < data.len() {
            let line = data[i]
                .split_ascii_whitespace()
                .map(String::from)
                .collect::<Vec<String>>();
            if !is_cmd {
                if line[0] == "$" {
                    ls_list.push(cur_ls_list.clone());
                    cur_ls_list.clear();
                    is_cmd = true;
                } else {
                    cur_ls_list.push(line.clone());
                    i += 1;
                }
            } else if line[0] == "$" {
                cmds.push(line[1..].iter().map(String::from).collect::<VS>().clone());
                i += 1;
            } else {
                is_cmd = false;
            }
        }
        if !is_cmd {
            ls_list.push(cur_ls_list.clone());
        }

        Self {
            cmds,
            ls_list,
            root: None,
            dir_sizes: vec![],
            max_size: 70000000,
            least_size_needed_to_delete: 0,
            cur_least_dir: None,
            cur_least: usize::MAX,
        }
    }

    pub fn execute_cmds(&mut self) {
        let root: RRIN = Rc::new(RefCell::new(INode::new(
            self.cmds[0][1].clone(),
            0,
            true,
            None,
        )));
        self.root = Some(root.clone());
        let mut node: RRIN = root.clone();
        let mut ls_i = 0;

        for cmd in self.cmds.iter().skip(1) {
            if cmd[0] == "cd" {
                if cmd[1] == "/" && node.borrow().name != "/" {
                    node = Rc::clone(&root);
                } else if cmd[1] == ".." {
                    let p = node.borrow().parent.as_ref().unwrap().clone();
                    node = p;
                } else {
                    // Get child in current directory.
                    let cc = node.borrow().children.clone();
                    for c in cc {
                        if c.borrow().name == cmd[1] {
                            node = Rc::clone(&c);
                            break;
                        }
                    }
                }
            } else if cmd[0] == "ls" {
                // Add children to current node.
                for n in self.ls_list[ls_i].clone() {
                    let mut size = 0;
                    let mut is_dir = true;
                    if n[0] != "dir" {
                        size = n[0].parse::<usize>().unwrap();
                        is_dir = false;
                    }
                    let child = Rc::new(RefCell::new(INode::new(
                        n[1].clone(),
                        size,
                        is_dir,
                        Some(node.clone()),
                    )));
                    node.borrow_mut().children.push(child);
                }
                ls_i += 1;
            } else {
                panic!("Unrecognized command.");
            }
        }
    }

    pub fn part1(&mut self, size: usize) {
        self.dir_sizes.clear();
        self.calculate_dir_sizes(self.root.as_ref().unwrap().clone());
        self.get_dir_sizes(size, self.root.as_ref().unwrap().clone());
        println!("Total sizes: {}", self.dir_sizes.iter().sum::<usize>());
    }

    fn calculate_dir_sizes(&mut self, node: RRIN) -> usize {
        if node.borrow().children.is_empty() {
            return node.borrow().size;
        }
        let mut total_size = 0;
        for c in &node.borrow().children {
            total_size += self.calculate_dir_sizes(c.clone());
        }
        node.borrow_mut().size = total_size;
        total_size
    }

    fn get_dir_sizes(&mut self, size: usize, node: RRIN) {
        if node.borrow().is_dir && node.borrow().size <= size {
            self.dir_sizes.push(node.borrow().size);
        }

        for c in &node.borrow().children {
            self.get_dir_sizes(size, c.clone());
        }
    }

    pub fn part2(&mut self) {
        self.least_size_needed_to_delete =
            30000000 - (self.max_size - self.root.as_ref().unwrap().borrow().size);
        self.find_least_size_dir(self.root.as_ref().unwrap().clone());
        println!(
            "Smallest dir: {} size {}",
            self.cur_least_dir.as_ref().unwrap().borrow().name,
            self.cur_least_dir.as_ref().unwrap().borrow().size
        );
    }

    fn find_least_size_dir(&mut self, node: RRIN) {
        for c in &node.borrow().children {
            self.find_least_size_dir(c.clone());
        }
        if node.borrow().is_dir
            && node.borrow().size > self.least_size_needed_to_delete
            && node.borrow().size < self.cur_least
        {
            self.cur_least = std::cmp::min(node.borrow().size, self.cur_least);
            self.cur_least_dir = Some(node.clone());
        }
    }
}

fn main() {
    let mut dir = Directory::new();
    dir.execute_cmds();
    dir.part1(100000);
    dir.part2();
}
