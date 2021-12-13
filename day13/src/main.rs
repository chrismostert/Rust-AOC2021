use std::collections::HashSet;
type Grid = HashSet<(usize, usize)>;

enum FoldInstruction {
    Up(usize),
    Left(usize),
}

fn make_grid(input: &str) -> Grid {
    input.lines().fold(HashSet::default(), |mut acc, x| {
        let (x, y) = x.split_once(',').unwrap();
        acc.insert((x.parse().unwrap(), y.parse().unwrap()));
        acc
    })
}

fn get_instructions(input: &'static str) -> impl Iterator<Item = FoldInstruction> {
    input.lines().map(|line| {
        let (t, v) = line
            .split_whitespace()
            .nth(2)
            .unwrap()
            .split_once('=')
            .unwrap();
        match t {
            "x" => FoldInstruction::Left(v.parse().unwrap()),
            "y" => FoldInstruction::Up(v.parse().unwrap()),
            _ => unreachable!(),
        }
    })
}

fn grid_size(grid: &Grid) -> (usize, usize) {
    grid.iter().fold((0, 0), |mut acc, x| {
        if x.0 + 1 > acc.0 {
            acc.0 = x.0 + 1;
        };
        if x.1 + 1 > acc.1 {
            acc.1 = x.1 + 1;
        };
        acc
    })
}

fn print_grid(grid: &Grid) {
    let (w, h) = grid_size(grid);
    for y in 0..h {
        for x in 0..w {
            match grid.get(&(x, y)) {
                Some(_) => print!("#"),
                None => print!(" "),
            }
        }
        println!();
    }
}

fn do_fold(grid: &mut Grid, instruction: FoldInstruction) {
    let (w, h) = grid_size(grid);
    match instruction {
        FoldInstruction::Up(line) => {
            for x in 0..w {
                for y in line + 1..h {
                    if grid.get(&(x, y)).is_some() {
                        grid.insert((x, line - (y - line)));
                        grid.remove(&(x, y));
                    }
                }
            }
        }
        FoldInstruction::Left(line) => {
            for x in line + 1..w {
                for y in 0..h {
                    if grid.get(&(x, y)).is_some() {
                        grid.insert((line - (x - line), y));
                        grid.remove(&(x, y));
                    }
                }
            }
        }
    }
}

fn main() {
    let (input, folds) = include_str!("../input.txt").split_once("\n\n").unwrap();
    let mut grid = make_grid(input);
    let instructions = get_instructions(folds);

    for (i, instruction) in instructions.enumerate() {
        do_fold(&mut grid, instruction);
        if i == 0 {
            println!("Part 1: {}", grid.len());
        }
    }
    println!("Part 2:");
    print_grid(&grid);
}
