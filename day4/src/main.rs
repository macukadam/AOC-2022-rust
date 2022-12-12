fn main() {
    let input = std::fs::read_to_string("4_input.txt").unwrap();
    let lines_sep: Vec<Vec<u8>> = input
        .lines()
        .map(|x| x.split(&['-', ',']).map(|x| x.parse().unwrap()).collect())
        .collect();

    let sum: i32 = lines_sep
        .iter()
        .map(|x| {
            (x[0]..=x[1]).all(|y| {
                (x[2]..=x[3]).contains(&y) || (x[2]..=x[3]).all(|y| (x[0]..=x[1]).contains(&y))
            }) as i32
        })
        .sum();
    println!("{:?}", sum);

    let sum: i32 = lines_sep
        .iter()
        .map(|x| (x[0]..=x[1]).any(|y| (x[2]..=x[3]).contains(&y)) as i32)
        .sum();
    println!("{:?}", sum);
}
