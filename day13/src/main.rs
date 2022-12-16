use std::{
    cmp::Ordering,
    fmt::{Debug, Display},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(untagged)]
enum Node<T> {
    Value(T),
    List(Vec<Node<T>>),
}

impl<T> std::cmp::Ord for Node<T>
where
    T: Eq + PartialOrd + Display + Debug,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<T> std::cmp::PartialOrd for Node<T>
where
    T: PartialEq + PartialOrd + Display + Debug,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Node::Value(x), Node::Value(y)) => x.partial_cmp(y),
            (Node::List(x), Node::List(y)) => x
                .iter()
                .zip(y.iter())
                .map(|(a, b)| a.partial_cmp(b))
                .find(|v| v.unwrap() != Ordering::Equal)
                .unwrap_or_else(|| Some(x.len().cmp(&y.len()))),
            (x, Node::List(y)) => [x]
                .into_iter()
                .zip(y.iter())
                .map(|(a, b)| a.partial_cmp(b))
                .find(|v| v.unwrap() != Ordering::Equal)
                .unwrap_or_else(|| Some(1.cmp(&y.len()))),
            (Node::List(x), y) => x
                .iter()
                .zip([y].iter())
                .map(|(a, b)| a.partial_cmp(b))
                .find(|v| v.unwrap() != Ordering::Equal)
                .unwrap_or_else(|| Some(x.len().cmp(&1))),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value(x) => write!(f, "{}", x),
            Self::List(x) => write!(f, "{:?}", x),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("13_input.txt").unwrap();

    let res = input
        .split("\n\n")
        .map(|s| {
            let v = s.split_once("\n").unwrap();
            (
                serde_json::from_str::<Node<i32>>(v.0).unwrap(),
                serde_json::from_str::<Node<i32>>(v.1).unwrap(),
            )
        })
        .enumerate()
        .fold(0, |mut acc, (i, (n1, n2))| {
            if n1 <= n2 {
                acc += i + 1
            }
            acc
        });

    // Answer 1
    println!("{}", res);

    let n1 = Node::List(vec![Node::Value(2)]);
    let n2 = Node::List(vec![Node::Value(6)]);

    let mut packets = std::fs::read_to_string("13_input.txt")
        .unwrap()
        .lines()
        .filter(|s| !s.is_empty())
        .map(|line| serde_json::from_str::<Node<i32>>(line).unwrap())
        .collect::<Vec<_>>();

    packets.push(n1.clone());
    packets.push(n2.clone());
    packets.sort();

    let res = (packets.binary_search(&n1).unwrap() + 1) * (packets.binary_search(&n2).unwrap() + 1);

    // Answer 2
    println!("{}", res);
}
