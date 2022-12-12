#![allow(dead_code)]

use std::str::FromStr;

#[derive(Debug)]
struct Tree {
    height: u8,
    visible: bool,
}

impl Tree {
    fn new(height: u8) -> Self {
        Self {
            height,
            visible: false,
        }
    }
}

#[derive(Debug)]
struct Forest {
    trees: Vec<Vec<Tree>>,
}

impl Forest {
    fn new() -> Self {
        Self { trees: vec![] }
    }

    fn get_visible_tree_count(&self) -> i32 {
        self.trees
            .iter()
            .map(|x| x.iter().map(|x| x.visible as i32).sum::<i32>())
            .sum()
    }

    fn get_max_score(&self) -> i32 {
        let mut max = 0;
        for i in 0..self.trees.len() {
            for j in 0..self.trees[i].len() {
                max = std::cmp::max(max, self.get_scenic_score(i, j));
            }
        }
        max
    }
    fn get_scenic_score(&self, row: usize, col: usize) -> i32 {
        let tree_height = &self.trees[row][col].height;

        let mut score = 1;
        let mut val = 0;
        for i in (0..col).rev() {
            if self.trees[row][i].height < *tree_height {
                val += 1;
            } else {
                val += 1;
                break;
            }
        }

        score *= val;
        val = 0;
        for i in col + 1..self.trees[row].len() {
            if self.trees[row][i].height < *tree_height {
                val += 1;
            } else {
                val += 1;
                break;
            }
        }

        let vertical = (0..self.trees.len()).fold(vec![], |mut acc: Vec<u8>, i| {
            acc.push(self.trees[i][col].height);
            acc
        });

        score *= val;
        val = 0;
        for i in (0..row).rev() {
            if vertical[i] < *tree_height {
                val += 1;
            } else {
                val += 1;
                break;
            }
        }

        score *= val;
        val = 0;
        for i in row + 1..self.trees.len() {
            if self.trees[i][col].height < *tree_height {
                val += 1;
            } else {
                val += 1;
                break;
            }
        }

        score *= val;
        score
    }

    fn tag_visible_trees(&mut self) {
        // tag left & right
        for j in 0..self.trees.len() {
            let len = self.trees[j].len();
            self.trees[j][0].visible = true;
            self.trees[j][len - 1].visible = true;
            for i in 1..len - 1 {
                if self.trees[j][0..i]
                    .iter()
                    .all(|x| x.height < self.trees[j][i].height)
                {
                    self.trees[j][i].visible = true;
                }

                if !self.trees[j][i].visible
                    && self.trees[j][i + 1..len]
                        .iter()
                        .rev()
                        .all(|x| x.height < self.trees[j][i].height)
                {
                    self.trees[j][i].visible = true;
                }
            }
        }

        // tag up & down
        for j in 0..self.trees[0].len() {
            let len = self.trees.len();
            let vertical = (0..len).fold(vec![], |mut acc: Vec<u8>, i| {
                acc.push(self.trees[i][j].height);
                acc
            });

            self.trees[0][j].visible = true;
            self.trees[len - 1][j].visible = true;

            for i in 1..len {
                if !self.trees[i][j].visible
                    && vertical[0..i].iter().all(|x| *x < self.trees[i][j].height)
                {
                    self.trees[i][j].visible = true;
                }

                if !self.trees[i][j].visible
                    && vertical[i + 1..]
                        .iter()
                        .all(|x| *x < self.trees[i][j].height)
                {
                    self.trees[i][j].visible = true;
                }
            }
        }
    }
}

impl FromStr for Forest {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trees = s
            .lines()
            .map(|x| {
                x.chars()
                    .map(|x| Tree::new(x.to_digit(10).unwrap() as u8))
                    .collect()
            })
            .collect();

        Ok(Forest { trees })
    }
}

fn main() {
    let input = std::fs::read_to_string("8_input.txt").unwrap();
    let mut forest = Forest::from_str(&input).unwrap();

    forest.tag_visible_trees();
    let sum = forest.get_visible_tree_count();
    println!("{:?}", sum);

    let score = forest.get_max_score();
    println!("{:?}", score);
}
