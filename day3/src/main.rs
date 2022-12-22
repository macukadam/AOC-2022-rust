fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let sum: i32 = input
        .lines()
        .map(|s| s.split_at(s.len() / 2))
        .map(|(l, r)| l.chars().find(|i| r.contains(*i)).unwrap())
        .into_iter()
        .map(|x| match x {
            'a'..='z' => x as i32 - 'a' as i32 + 1,
            _ => x as i32 - 'A' as i32 + 27,
        })
        .sum();

    println!("{:?}", sum);

    let sum: Vec<Vec<&str>> = input
        .lines()
        .collect::<Vec<&str>>()
        .as_slice()
        .chunks(3)
        .map(|x| x.to_vec())
        .collect();

    let mut vals: Vec<i32> = vec![];
    for s in &sum {
        'loop1: for v in s {
            for c in v.chars() {
                if s[1].contains(c) && s[2].contains(c) {
                    let a = match c {
                        'a'..='z' => c as i32 - 'a' as i32 + 1,
                        _ => c as i32 - 'A' as i32 + 27,
                    };

                    vals.push(a);
                    break 'loop1;
                }
            }
        }
    }

    println!("{:?}", vals.iter().sum::<i32>());
}
