use std::{cell::RefCell, error::Error, fmt::Display, rc::Rc};
type REFDIR = Rc<RefCell<Directory>>;

enum Command {
    CD(String),
    DIR(Directory),
    FILE(File),
    LS,
}

impl std::str::FromStr for Command {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cmd: Vec<&str> = s.split(" ").collect();
        match cmd[0] {
            "$" => match cmd[1] {
                "cd" => Ok(Command::CD(cmd[2].to_string())),
                "ls" => Ok(Command::LS),
                _ => Err("Not found".into()),
            },
            "dir" => Ok(Command::DIR(Directory::new(cmd[1].to_string()))),
            _ => Ok(Command::FILE(File::new(
                cmd[1].to_string(),
                cmd[0].parse().unwrap(),
            ))),
        }
    }
}

struct File {
    _name: String,
    size: i32,
}

impl File {
    fn new(_name: String, size: i32) -> Self {
        Self { _name, size }
    }
}

struct Directory {
    name: String,
    files: Vec<File>,
    dirs: Vec<REFDIR>,
    parent: Option<REFDIR>,
}

impl Display for Directory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = write!(
            f,
            "({}-{})",
            self.name,
            self.dirs
                .iter()
                .map(|x| x.clone().borrow().name.clone())
                .collect::<Vec<String>>()
                .join(" ")
        );
        str
    }
}

impl Directory {
    fn new(name: String) -> Self {
        Directory {
            name,
            files: vec![],
            dirs: vec![],
            parent: None,
        }
    }

    fn add_dir(&mut self, dir: REFDIR) {
        self.dirs.push(dir);
    }
}

fn main() {
    let input = std::fs::read_to_string("7_input.txt").unwrap();
    let lines = input.lines();
    let seperated: Vec<&str> = lines.collect();

    let mapped: Vec<Command> = seperated
        .iter()
        .map(|x| x.parse::<Command>().unwrap())
        .collect();

    let root = Rc::new(RefCell::new(Directory::new("/".to_string())));
    let mut node = root.clone();

    for m in mapped {
        match m {
            Command::CD(dir_name) => match dir_name.as_str() {
                "/" => {}
                ".." => {
                    let parent = node.borrow().parent.clone().unwrap();
                    node = parent;
                }
                _ => {
                    let parent = node.borrow().dirs.clone();
                    node = parent
                        .iter()
                        .find(|x| x.borrow().name == dir_name)
                        .unwrap()
                        .clone();
                }
            },
            Command::FILE(file) => {
                node.borrow_mut().files.push(file);
            }
            Command::DIR(mut dir) => {
                dir.parent = Some(node.clone());
                node.borrow_mut().add_dir(REFDIR::new(RefCell::new(dir)));
            }
            Command::LS => {}
        }
    }

    let mut all_sums: Vec<i32> = vec![];
    let sum = total_size(root, &mut all_sums);

    // Answer 1
    println!("{}", all_sums.iter().filter(|x| **x < 100000).sum::<i32>());

    let space_neded = 30000000 - (70000000 - sum);
    let mut filtered: Vec<&i32> = all_sums.iter().filter(|x| **x > space_neded).collect();
    filtered.sort();

    // Answer 2
    println!("{}", filtered[0]);
}

fn total_size(dir: REFDIR, all_sums: &mut Vec<i32>) -> i32 {
    let mut sum: i32 = dir.borrow().files.iter().map(|x| x.size).sum();

    sum += dir.borrow().dirs.iter().fold(0, |acc, child_dir| {
        acc + total_size(child_dir.clone(), all_sums)
    });

    all_sums.push(sum);
    sum
}
