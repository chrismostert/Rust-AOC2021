type Coordinate = (usize, usize);
type Grid = Vec<Vec<Octopus>>;

enum Octopus {
    Charging(u32),
    Exploded,
}

fn parse_input(input: &str) -> Grid {
    input.lines().fold(Vec::new(), |mut outer, line| {
        let row = line.chars().fold(Vec::new(), |mut inner, digit| {
            inner.push(Octopus::Charging(digit.to_digit(10).unwrap()));
            inner
        });
        outer.push(row);
        outer
    })
}

fn neighbors(input: &[Vec<Octopus>], coord: Coordinate) -> Vec<Coordinate> {
    let (x, y) = coord;
    let mut res = Vec::new();

    for i in x.saturating_sub(1)..=x + 1 {
        for j in y.saturating_sub(1)..=y + 1 {
            if let Some(row) = input.get(i) {
                if row.get(j).is_some() & ((x, y) != (i, j)) {
                    res.push((i, j));
                }
            }
        }
    }

    res
}

fn do_step(input: &mut Grid) -> usize {
    let mut igniting = Vec::default();
    let mut ignitions = 0;
    let len = input.len();

    // Increase energy by 1
    for row in input.iter_mut() {
        for octopus in row {
            if let Octopus::Charging(v) = octopus {
                *octopus = Octopus::Charging(*v + 1);
            };
        }
    }

    // Check ignitions
    loop {
        for row in 0..len {
            for col in 0..len {
                // Ignite?
                if let Octopus::Charging(v) = input[row][col] {
                    if v > 9 {
                        igniting.push((row, col));
                    }
                }
            }
        }

        if igniting.is_empty() {
            break;
        }

        for (x, y) in &igniting {
            input[*x][*y] = Octopus::Exploded;
            ignitions += 1;
            for (n_r, n_c) in neighbors(input, (*x, *y)) {
                if let Octopus::Charging(v) = input[n_r][n_c] {
                    input[n_r][n_c] = Octopus::Charging(v + 1);
                }
            }
        }

        igniting = Vec::default();
    }

    // Reset all ignitied octopi
    for row in input.iter_mut() {
        for octopus in row {
            if let Octopus::Exploded = octopus {
                *octopus = Octopus::Charging(0);
            }
        }
    }

    ignitions
}

fn main() {
    let mut input = parse_input(include_str!("../input.txt"));
    let p1 = (0..100).fold(0, |acc, _| acc + do_step(&mut input));

    input = parse_input(include_str!("../input.txt"));
    let mut res = 0;
    let mut steps = 0;
    while res != 100 {
        steps += 1;
        res = do_step(&mut input);
    }

    println!("Part 1: {}", p1);
    println!("Part 2: {}", steps);
}
