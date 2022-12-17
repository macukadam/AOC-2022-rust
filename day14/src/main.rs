use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("14_input.txt").unwrap();
    let mut cave: HashSet<(i32, i32)> = HashSet::new();
    let mut max_global_y = i32::MIN;

    // possibly can be shorter?
    input.lines().for_each(|line| {
        let split: Vec<(i32, i32)> = line
            .split(" ")
            .step_by(2)
            .map(|x| x.split_once(","))
            .flatten()
            .map(|x| (x.0.parse().unwrap(), x.1.parse().unwrap()))
            .collect();

        split[..split.len() - 1]
            .iter()
            .enumerate()
            .for_each(|(i, _)| {
                let (x, y) = split[i];
                let (x1, y1) = split[i + 1];

                let min_y = y.min(y1);
                let max_y = y.max(y1);
                let min_x = x.min(x1);
                let max_x = x.max(x1);
                max_global_y = max_global_y.max(max_y);

                if max_y == min_y {
                    for i in min_x..=max_x {
                        cave.insert((i, y));
                    }
                }

                if max_x == min_x {
                    for i in min_y..=max_y {
                        cave.insert((x, i));
                    }
                }
            });
    });

    let mut sum = 0;
    while sand_down(&mut cave, 500, 0, max_global_y) {
        sum += 1;
    }

    // Answer 1
    println!("{sum}");

    while sand_down_with_bottom(&mut cave, 500, 0, max_global_y) {
        sum += 1;
    }

    // Answer 2
    println!("{sum}");
}

fn sand_down_with_bottom(cave: &mut HashSet<(i32, i32)>, x: i32, y: i32, max_global_y: i32) -> bool {
    if cave.contains(&(500, 0)) {
        return false;
    }

    // in case of the max y (sand or rock) is reached add new layer of to the bottom
    if y >= max_global_y {
        let diff = max_global_y - y + 2;
        cave.insert((x, y + diff));
        cave.insert((x + 1, y + diff));
        cave.insert((x - 1, y + diff));
    }

    if !cave.contains(&(x, y + 1)) {
        return sand_down_with_bottom(cave, x, y + 1, max_global_y);
    } else {
        if !cave.contains(&(x - 1, y + 1)) {
            return sand_down_with_bottom(cave, x - 1, y + 1, max_global_y);
        } else if !cave.contains(&(x + 1, y + 1)) {
            return sand_down_with_bottom(cave, x + 1, y + 1, max_global_y);
        } else {
            cave.insert((x, y));
            return true
        }
    }
}

// rampapa pam rampapa pam ram papapaaaaam
fn sand_down(cave: &mut HashSet<(i32, i32)>, x: i32, y: i32, max_global_y: i32) -> bool {
    if y > max_global_y {
        return false;
    }

    // immediate bottom check
    if !cave.contains(&(x, y + 1)) {
        return sand_down(cave, x, y + 1, max_global_y);
    } else {
        // left bottom check
        if !cave.contains(&(x - 1, y + 1)) {
            return sand_down(cave, x - 1, y + 1, max_global_y);
        // right bottom check
        } else if !cave.contains(&(x + 1, y + 1)) {
            return sand_down(cave, x + 1, y + 1, max_global_y);
        // insert if nowhere to go down
        } else {
            cave.insert((x, y));
            return true;
        }
    }
}
