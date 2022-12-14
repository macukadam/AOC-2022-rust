use std::{fmt::Display, str::FromStr};

struct Monkey {
    name: i128,
    items: Vec<i128>,
    operation: Box<dyn Fn(i128) -> i128>,
    test: Box<dyn Fn(i128) -> i128>,
    inspect_count: i128,
    divisable: i128,
}

struct KeepAway<'a> {
    monkeys: &'a mut Vec<Monkey>,
    comon_divisable: i128,
}

impl<'a> KeepAway<'a> {
    fn new(monkeys: &'a mut Vec<Monkey>) -> Self {
        let comon_divisable = monkeys.iter().map(|monkey| monkey.divisable).product();
        Self {
            monkeys,
            comon_divisable,
        }
    }

    fn run_one_turn(&mut self, divider: Option<i128>) {
        for i in 0..self.monkeys.len() {
            for _ in 0..self.monkeys[i].items.len() {
                let item_worry = self.monkeys[i].items.remove(0);
                let mut item_new_worry = (self.monkeys[i].operation)(item_worry);

                if let Some(x) = divider {
                    item_new_worry /= x;
                } else {
                    item_new_worry %= self.comon_divisable;
                }
                let monkey_index = (self.monkeys[i].test)(item_new_worry) as usize;

                self.monkeys[i].inspect_count += 1;
                self.monkeys[monkey_index].items.push(item_new_worry);
            }
        }
    }

    fn get_result(&self) -> i128 {
        let mut inspected_counts = self
            .monkeys
            .iter()
            .map(|x| x.inspect_count)
            .collect::<Vec<i128>>();

        inspected_counts.sort();
        inspected_counts.reverse();
        inspected_counts[0] * inspected_counts[1]
    }
}

impl Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Monkey {} has items {:?} inspected_count: {}.",
            self.name, self.items, self.inspect_count
        )
    }
}

impl FromStr for Monkey {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split("\n").collect();
        let (_, line) = split[0].split_once(" ").unwrap();
        let name: i128 = line[..line.len() - 1].parse().unwrap();

        let items: Vec<i128> = split[1]
            .split(|x| x == ' ' || x == ',')
            .filter_map(|x| x.trim().parse().ok())
            .collect();

        let ops: Vec<&str> = split[2].split("=").skip(1).take(1).collect::<Vec<&str>>()[0]
            .split(" ")
            .skip(1)
            .collect::<Vec<&str>>();

        let operation: Box<dyn Fn(i128) -> i128> = match ops[1] {
            "+" => match (ops[0].parse::<i128>(), ops[2].parse::<i128>()) {
                (Ok(x), Err(_)) | (Err(_), Ok(x)) => Box::new(move |old| old + x),
                (Err(_), Err(_)) => Box::new(move |old| old + old),
                _ => panic!(),
            },
            "*" => match (ops[0].parse::<i128>(), ops[2].parse::<i128>()) {
                (Ok(x), Err(_)) | (Err(_), Ok(x)) => Box::new(move |old| old * x),
                (Err(_), Err(_)) => Box::new(move |old| old * old),
                _ => panic!(),
            },
            _ => panic!(),
        };

        let divisable = split[3]
            .split(" ")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .parse::<i128>()
            .unwrap();

        let true_monkey_index = split[4]
            .split(" ")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .parse::<i128>()
            .unwrap();

        let false_monkey_index = split[5]
            .split(" ")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .parse::<i128>()
            .unwrap();

        let test = Box::new(move |value: i128| -> i128 {
            if value % divisable == 0 {
                return true_monkey_index;
            }
            false_monkey_index
        });

        let inspect_count = 0;

        Ok(Self {
            name,
            items,
            operation,
            test,
            inspect_count,
            divisable,
        })
    }
}

fn main() {
    let input = std::fs::read_to_string("11_input.txt").unwrap();
    let split: Vec<&str> = input.split("\n\n").collect();
    let mut monkeys: Vec<Monkey> = split.iter().map(|x| x.parse().unwrap()).collect();
    let mut game = KeepAway::new(&mut monkeys);
    (0..20).for_each(|_| game.run_one_turn(Some(3)));
    let i = game.get_result();

    println!("{}", i);

    let mut monkeys: Vec<Monkey> = split.iter().map(|x| x.parse().unwrap()).collect();
    let mut game = KeepAway::new(&mut monkeys);
    (0..10_000).for_each(|_| game.run_one_turn(None));
    let i = game.get_result();

    // Answer 2:
    println!("{}", i);
}
