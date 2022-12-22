fn main() {
    let mut split: Vec<i32> = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .collect::<Vec<&str>>()
        .split(|x| x == &"")
        .map(|x| x.iter().map(|x| x.parse::<i32>().unwrap()).sum())
        .collect();
    split.sort();

    // first part
    println!("{:?}", split.iter().rev().take(1).sum::<i32>());
    // second part
    println!("{:?}", split.iter().rev().take(3).sum::<i32>());
}
