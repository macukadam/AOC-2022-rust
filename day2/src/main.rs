mod game;

use game::Game;
use game::GameResult;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines: Vec<(&str, &str)> = input.lines().map(|x| x.split_once(" ").unwrap()).collect();
    let result: u32 = lines
        .iter()
        .map(|(x, y)| Game::new(y, x).get_result())
        .sum();

    println!("{}", result);

    let result: u32 = lines
        .iter()
        .map(|(x, y)| Game::new_two(y, x).get_result())
        .sum();

    println!("{}", result);
}
