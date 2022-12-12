use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("6_input.txt").unwrap();
    let answer1 = get_unique_index(&input, 4);

    println!("{}", answer1);
    let answer2 = get_unique_index(&input, 14);
    println!("{}", answer2);
}

fn get_unique_index(input: &str, length: usize) -> i32 {
    let mut result = 0;
    for i in 0..input.len() {
        let s = &input[i..i + length];
        let hashed = s.chars().fold(HashSet::new(), |mut acc: HashSet<char>, c| {
            acc.insert(c);
            acc
        });

        if hashed.len() == length{
            result = i as i32 + length as i32;
            break;
        }
    }
    result as i32
}
