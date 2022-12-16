#![allow(dead_code)]
use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

struct Map {
    grid: Vec<Vec<Node>>,
    visited: HashSet<(usize, usize)>,
    start_pos: Node,
    end_pos: Node,
}

impl Map {
    fn reset_visited(&mut self) {
        self.visited = HashSet::new();
    }
    fn get_node(&mut self, x: i32, y: i32) -> Option<Node> {
        if x >= 0 && x < self.grid.len() as i32 && y >= 0 && y < self.grid[0].len() as i32 {
            return Some(self.grid[x as usize][y as usize]);
        }
        None
    }

    fn get_starting_positions(&self) -> Vec<&Node> {
        self.grid
            .iter()
            .map(|x| {
                x.iter()
                    .filter_map(|x| (x.h == 'a' as u8).then(|| x))
                    .collect()
            })
            .collect::<Vec<Vec<&Node>>>()
            .concat()
    }

    fn bfs_for_each(&mut self, starting_positions: Vec<&Node>) -> i32 {
        let mut vals: Vec<i32> = vec![];

        for sp in starting_positions {
            self.reset_visited();
            self.start_pos = *sp;
            vals.push(self.bfs());
        }

        let mut rems = vals.iter().filter(|x| **x != -1).collect::<Vec<&i32>>();
        rems.sort();
        *rems[0]
    }
}

#[derive(Default, Clone, Copy, Debug)]
struct Node {
    x: usize,
    y: usize,
    h: u8,
}

impl Map {
    fn bfs(&mut self) -> i32 {
        let mut nodes: VecDeque<(Node, i32)> = VecDeque::new();
        nodes.push_back((self.start_pos, 0));

        while !nodes.is_empty() {
            let val = nodes.pop_front().unwrap();

            let Node { x, y, h, .. } = val.0;

            if h == 'z' as u8 + 1 {
                return val.1;
            }

            let mut steps = val.1;

            if h != 'a' as u8 {
                steps += 1;
            }

            if let Some(node) = self.get_node(x as i32 + 1, y as i32) {
                if !self.visited.contains(&(x, y)) && (node.h <= h + 1) {
                    nodes.push_back((node, steps));
                }
            }

            if let Some(node) = self.get_node(x as i32, y as i32 + 1) {
                if !self.visited.contains(&(x, y)) && (node.h <= h + 1) {
                    nodes.push_back((node, steps));
                }
            }

            if let Some(node) = self.get_node(x as i32 - 1, y as i32) {
                if !self.visited.contains(&(x, y)) && (node.h <= h + 1) {
                    nodes.push_back((node, steps));
                }
            }

            if let Some(node) = self.get_node(x as i32, y as i32 - 1) {
                if !self.visited.contains(&(x, y)) && (node.h <= h + 1) {
                    nodes.push_back((node, steps));
                }
            }

            self.visited.insert((x, y));
        }
        -1
    }
}

impl FromStr for Map {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start_pos = Node::default();
        let mut end_pos = Node::default();
        let grid = s
            .lines()
            .enumerate()
            .map(|(x, c)| {
                c.chars()
                    .enumerate()
                    .map(|(y, c)| match c {
                        'S' => {
                            start_pos = Node { x, y, h: 'a' as u8 };
                            start_pos
                        }
                        'E' => {
                            end_pos = Node {
                                x,
                                y,
                                h: 'z' as u8 + 1,
                            };
                            end_pos
                        }
                        _ => Node { x, y, h: c as u8 },
                    })
                    .collect()
            })
            .collect();

        Ok(Self {
            grid,
            start_pos,
            end_pos,
            visited: HashSet::new(),
        })
    }
}

fn main() {
    let input = std::fs::read_to_string("12_input.txt").unwrap();
    let mut map: Map = input.parse().unwrap();
    // Answer 1
    println!("{}", map.bfs());

    // Answer 2
    let starting_positions = map.get_starting_positions();
    let mut map: Map = input.parse().unwrap();
    println!("{:?}", map.bfs_for_each(starting_positions));
}
