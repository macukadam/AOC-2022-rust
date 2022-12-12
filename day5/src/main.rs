fn main() {
    let input = std::fs::read_to_string("5_input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let splited = lines.split_at(10);

    let crate_part = &splited.0[..splited.0.len() - 2];

    let crates: Vec<Vec<char>> = crate_part
        .iter()
        .map(|x| x.chars().skip(1).step_by(4).collect())
        .collect();

    let mut transposed = (0..=crates.len()).fold(vec![], |mut acc: Vec<Vec<char>>, i| {
        acc.push(
            crates
                .iter()
                .filter_map(|x| (!x[i].is_whitespace()).then_some(x[i]))
                .rev()
                .collect(),
        );
        acc
    });

    let move_part = &splited.1;
    let moves: Vec<Vec<usize>> = move_part
        .iter()
        .map(|x| {
            x.split(' ')
                .skip(1)
                .step_by(2)
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect();

    let mut t1 = transposed.clone();
    let m1 = moves.clone();
    for m in m1 {
        for _ in 0..m[0] {
            let poped = t1[m[1] - 1].pop().unwrap();
            t1[m[2] - 1].push(poped);
        }
    }

    // Answer 1
    print!("A1: ");
    for t in t1 {
        print!("{}", t.last().unwrap());
    }

    println!("");

    for m in moves {
        let from = m[1] - 1;
        let to = m[2] - 1;
        let amount = m[0];
        let len = transposed[from].len();
        let mut drained = transposed[from].split_off(len - amount);
        transposed[to].append(&mut drained);
    }

    // Answer 2
    print!("A2: ");
    for t in transposed {
        print!("{}", t.last().unwrap());
    }
}
