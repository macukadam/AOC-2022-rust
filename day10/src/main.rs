use std::str::FromStr;

enum Command {
    ADDX(i32),
    NOOP,
}

impl FromStr for Command {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split_once(" ").unwrap_or((s, ""));

        match split.0 {
            "addx" => Ok(Command::ADDX(split.1.parse().unwrap())),
            "noop" => Ok(Command::NOOP),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Wrong input",
            )),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("10_input.txt").unwrap();
    let instructions: Vec<Command> = input
        .lines()
        .map(|x| Command::from_str(x).unwrap())
        .collect();

    let mut x = 1;
    let mut results: Vec<i32> = vec![];

    instructions.iter().for_each(|command| match command {
        Command::ADDX(val) => {
            results.push(x);
            results.push(x);
            x += val;
        }
        Command::NOOP => {
            results.push(x);
        }
    });

    let sum = results
        .iter()
        .skip(19)
        .step_by(40)
        .enumerate()
        .fold(0, |acc, (i, res)| acc + (i as i32 * 40 + 20) * res);

    println!("{}", sum);

    let mut chars: Vec<char> = vec![];

    for (i, v) in results.iter().enumerate() {
        let sprite = [-1 + v, 0 + v, 1 + v].to_vec();

        if i % 40 == 0  {
            chars.push('\n');
        }

        if sprite.contains(&(i as i32 % 40)) {
            chars.push('#');
        } else {
            chars.push('.');
        }
    }

    let res: String = chars.iter().collect();
    println!("{}", res);
}
