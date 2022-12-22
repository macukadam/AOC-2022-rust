use std::{fmt::Display, str::FromStr};

enum RPC {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
struct RPCParseError;
impl std::error::Error for RPCParseError {}

impl Display for RPCParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Could not parse RPC.")
    }
}

impl FromStr for RPC {
    type Err = RPCParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(RPC::Rock),
            "B" => Ok(RPC::Paper),
            "C" => Ok(RPC::Scissors),
            "Y" => Ok(RPC::Paper),
            "X" => Ok(RPC::Rock),
            "Z" => Ok(RPC::Scissors),
            _ => Err(RPCParseError),
        }
    }
}

enum Outcome {
    Won,
    Lost,
    Draw,
}

impl RPC {
    fn points(&self) -> u32 {
        match self {
            RPC::Rock => 1,
            RPC::Paper => 2,
            RPC::Scissors => 3,
        }
    }
}

pub trait GameResult {
    fn get_result(&self) -> u32;
}

pub struct Game {
    you: RPC,
    enemy: RPC,
}

impl Game {
    pub fn new(you: &str, enemy: &str) -> Self {
        Self {
            you: RPC::from_str(you).unwrap(),
            enemy: RPC::from_str(enemy).unwrap(),
        }
    }

    pub fn new_two(you: &str, enemy: &str) -> Self {
        let your = match enemy {
            "A" => match you {
                "X" => Ok(RPC::Scissors),
                "Y" => Ok(RPC::Rock),
                "Z" => Ok(RPC::Paper),
                _ => Err(RPCParseError),
            },
            "B" => match you {
                "X" => Ok(RPC::Rock),
                "Y" => Ok(RPC::Paper),
                "Z" => Ok(RPC::Scissors),
                _ => Err(RPCParseError),
            },
            "C" => match you {
                "X" => Ok(RPC::Paper),
                "Y" => Ok(RPC::Scissors),
                "Z" => Ok(RPC::Rock),
                _ => Err(RPCParseError),
            },
            _ => Err(RPCParseError),
        };

        Self {
            you: your.unwrap(),
            enemy: RPC::from_str(enemy).unwrap(),
        }
    }
}

impl Game {
    fn get_outcome(&self) -> Outcome {
        match self.you {
            RPC::Rock => match self.enemy {
                RPC::Rock => Outcome::Draw,
                RPC::Paper => Outcome::Lost,
                RPC::Scissors => Outcome::Won,
            },
            RPC::Paper => match self.enemy {
                RPC::Rock => Outcome::Won,
                RPC::Paper => Outcome::Draw,
                RPC::Scissors => Outcome::Lost,
            },
            RPC::Scissors => match self.enemy {
                RPC::Rock => Outcome::Lost,
                RPC::Paper => Outcome::Won,
                RPC::Scissors => Outcome::Draw,
            },
        }
    }
}

impl GameResult for Game {
    fn get_result(&self) -> u32 {
        match self.get_outcome() {
            Outcome::Won => 6 + self.you.points(),
            Outcome::Draw => 3 + self.you.points(),
            Outcome::Lost => 0 + self.you.points(),
        }
    }
}
