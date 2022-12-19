fn main() {
    let input = std::fs::read_to_string("15_input.txt").unwrap();

    // Answer 1
    let sum = part_1(&input, 2_000_000);
    println!("{sum}");

    // Answer 2
    let sum = part_2(&input, 4_000_000);
    println!("{}", sum.unwrap());
}

fn parse_input(input: &str) -> Vec<((i64, i64), (i64, i64))> {
    input
        .lines()
        .map(|x| x.split(&[' ', ',', ':', '=']).collect())
        .map(|c: Vec<&str>| {
            (
                (c[3].parse().unwrap(), c[6].parse().unwrap()),
                (c[13].parse().unwrap(), c[16].parse().unwrap()),
            )
        })
        .collect::<Vec<_>>()
}

fn part_1(input: &str, line_index: i64) -> i64 {
    let res = parse_input(input).into_iter().fold(
        [i64::MAX, i64::MIN],
        |mut acc: [i64; 2], v: ((i64, i64), (i64, i64))| {
            let ((x, y), (x1, y1)) = v;
            let diff = (x - x1).abs() + (y - y1).abs();
            let x_begin = x - diff + (line_index - y).abs();
            let x_end = x + diff - (line_index - y).abs();
            acc = [x_begin.min(acc[0]), x_end.max(acc[1])];
            acc
        },
    );
    res[1] - res[0]
}

pub fn part_2(input: &str, limit: i64) -> Option<i64> {
    let vals = parse_input(input);
    (0..limit).fold(None, |acc: Option<i64>, i| {
        let out = vals
            .iter()
            .fold(vec![], |mut out: Vec<(i64, i64)>, ((x, y), (x1, y1))| {
                let diff = (x - x1).abs() + (y - y1).abs();
                let cs = diff - (y - i).abs();
                if cs >= 0 {
                    out.push((x - cs, x + cs));
                }
                out.sort();
                out
            });

        let (_, mut y) = out[0];
        for p in &out[1..] {
            if p.0 >= y + 1 {
                return Some((y + 1) * 4_000_000 + i);
            }
            y = y.max(p.1)
        }
        acc
    })
}
