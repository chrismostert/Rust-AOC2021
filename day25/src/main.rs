type Seafloor = Vec<Vec<Option<Cucumber>>>;
#[derive(Clone, Copy, PartialEq)]
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
    let mut changed = false;
    let order = [Cucumber::East, Cucumber::South];

    for cucumber in order {
        let prev = floor.clone();
        for y in 0..height {
            for x in 0..width {
                let (y_check, x_check) = match cucumber {
                    Cucumber::East => (y, if x == 0 { width - 1 } else { x - 1 }),
                    Cucumber::South => (if y == 0 { height - 1 } else { y - 1 }, x),
                };
                if prev[y_check][x_check] == Some(cucumber) && prev[y][x].is_none() {
                    floor[y][x] = Some(cucumber);
                    floor[y_check][x_check] = None;
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
