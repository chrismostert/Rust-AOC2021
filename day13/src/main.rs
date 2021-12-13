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
    grid.iter().fold((0, 0), |(a, b), (x, y)| {
        (Ord::max(a, x + 1), Ord::max(b, y + 1))
    })
}

fn print_grid(grid: &Grid) {
    let (w, h) = grid_size(grid);
    for y in 0..h {
        for x in 0..w {
            match grid.get(&(x, y)) {
                Some(_) => print!("⬜"),
                None => print!("⬛"),
            }
        }
        println!();
    }
}

fn do_fold(grid: &mut Grid, instruction: FoldInstruction) {
    let vals: Vec<(usize, usize)> = grid.iter().cloned().collect();
    match instruction {
        FoldInstruction::Up(line) => {
            for (x, y) in vals {
                if y > line {
                    grid.insert((x, line - (y - line)));
                    grid.remove(&(x, y));
                }
            }
        }
        FoldInstruction::Left(line) => {
            for (x, y) in vals {
                if x > line {
                    grid.insert((line - (x - line), y));
                    grid.remove(&(x, y));
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
