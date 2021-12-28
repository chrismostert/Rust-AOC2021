type Seafloor = Vec<Vec<Option<Cucumber>>>;
#[derive(Debug, Clone, Copy)]
enum Cucumber {
    East,
    South,
}

fn parse_input(input: &str) -> Seafloor {
    input.lines().fold(Vec::default(), |mut res, line| {
        res.push(line.chars().fold(Vec::default(), |mut acc, c| {
            match c {
                '>' => acc.push(Some(Cucumber::East)),
                'v' => acc.push(Some(Cucumber::South)),
                _ => acc.push(None),
            }
            acc
        }));
        res
    })
}

fn do_step(floor: &mut Seafloor, width: usize, height: usize) -> bool {
    let prev = floor.clone();
    let mut changed = false;

    // East pass
    for i in 0..height {
        for j in 0..width {
            if let Some(Cucumber::East) = prev[i][if j == 0 { width - 1 } else { j - 1 }] {
                if prev[i][j].is_none() {
                    floor[i][j] = Some(Cucumber::East);
                    floor[i][if j == 0 { width - 1 } else { j - 1 }] = None;
                    changed = true;
                }
            }
        }
    }
    // South pass
    let prev = floor.clone();
    for i in 0..height {
        for j in 0..width {
            if let Some(Cucumber::South) = prev[if i == 0 { height - 1 } else { i - 1 }][j] {
                if prev[i][j].is_none() {
                    floor[i][j] = Some(Cucumber::South);
                    floor[if i == 0 { height - 1 } else { i - 1 }][j] = None;
                    changed = true;
                }
            }
        }
    }

    changed
}

fn main() {
    let mut input = parse_input(include_str!("../input.txt"));
    let (width, height) = (input[0].len(), input.len());

    let mut count = 1;
    while do_step(&mut input, width, height) {
        count += 1;
    }

    println!("Part 1: {}", count);
    println!("Part 2: A christmas present 🎅 (there is no part 2)");
}
