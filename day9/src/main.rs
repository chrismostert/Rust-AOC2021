use itertools::sorted;
use std::collections::HashSet;

type Coordinate = (usize, usize);
type Grid = Vec<Vec<u32>>;

fn parse_input(input: &str) -> Grid {
    input.lines().fold(Vec::new(), |mut outer, line| {
        let row = line.chars().fold(Vec::new(), |mut inner, digit| {
            inner.push(digit.to_digit(10).unwrap());
            inner
        });
        outer.push(row);
        outer
    })
}

fn solve(input: &Grid) -> (u32, usize) {
    let mut p1 = 0;
    let mut basins: Vec<usize> = Vec::new();

    for (i, row) in input.iter().enumerate() {
        for (j, elem) in row.iter().enumerate() {
            if is_low(input, (i, j)) {
                p1 += 1 + elem;
                basins.push(basin_size(input, (i, j)));
            }
        }
    }
    (
        p1,
        sorted(basins.iter()).rev().take(3).product(),
    )
}

fn basin_size(input: &Grid, coord: Coordinate) -> usize {
    let mut res = 0;

    let mut visited: HashSet<Coordinate> = HashSet::default();
    let mut visiting = Vec::default();

    visiting.push(coord);

    while let Some(c) = visiting.pop() {
        if visited.contains(&c) {
            continue;
        }
        res += 1;
        visited.insert(c);

        visiting.extend(neighbors(input, c).iter().filter(|&x| input[x.0][x.1] < 9));
    }

    res
}

fn is_low(input: &Grid, coord: Coordinate) -> bool {
    let n = neighbors(input, coord);
    let res = n
        .iter()
        .map(|(x, y)| (input[*x][*y]))
        .filter(|num| (num > &input[coord.0][coord.1]))
        .count()
        == n.len();
    res
}

fn neighbors(input: &Grid, coord: Coordinate) -> Vec<Coordinate> {
    let (i, j) = coord;
    let mut res = Vec::new();
    for (x, y) in [
        (i.saturating_sub(1), j),
        (i + 1, j),
        (i, j.saturating_sub(1)),
        (i, j + 1),
    ] {
        if let Some(row) = input.get(x) {
            if row.get(y).is_some() & ((x != i) | (y != j)) {
                res.push((x, y));
            }
        }
    }
    res
}

fn main() {
    let input = parse_input(include_str!("../input.txt"));
    let (p1, p2) = solve(&input);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
