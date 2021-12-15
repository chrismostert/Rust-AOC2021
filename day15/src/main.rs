use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
type Grid = HashMap<(usize, usize), u32>;

fn neighbors(grid: &Grid, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
    [
        (x.saturating_sub(1), y),
        (x + 1, y),
        (x, y.saturating_sub(1)),
        (x, y + 1),
    ]
    .iter()
    .filter_map(|(i, j)| {
        if grid.get(&(*i, *j)).is_some() && (x != *i || y != *j) {
            return Some((*i, *j));
        }
        None
    })
    .collect::<Vec<_>>()
}

fn dijkstra(grid: &Grid) -> u32 {
    let mut dist: HashMap<(usize, usize), u32> = grid.keys().map(|&k| (k, u32::MAX)).collect();
    let mut frontier = BinaryHeap::new();
    let dest = *grid.keys().max().unwrap();

    *dist.get_mut(&(0, 0)).unwrap() = 0;
    frontier.push(Reverse((0, (0, 0))));

    while let Some(Reverse((cost, coordinate))) = frontier.pop() {
        if coordinate == dest {
            return cost;
        }
        if cost > dist[&coordinate] {
            continue;
        }
        for n in neighbors(grid, coordinate) {
            let next_cost = cost + grid[&n];
            let next = Reverse((cost + grid[&n], n));
            if next_cost < dist[&n] {
                frontier.push(next);
                *dist.get_mut(&n).unwrap() = next_cost;
            }
        }
    }
    0
}

fn expand(grid: &mut Grid, times: usize) {
    let (mut w, mut h) = *grid.keys().max().unwrap();
    w += 1;
    h += 1;
    for x in 0..(w * times) {
        for y in 0..(h * times) {
            let dx = (x / w) as u32;
            let dy = (y / h) as u32;
            let val = grid[&(x % w, y % h)] + dx + dy;
            *grid.entry((x, y)).or_default() = val % 10 + val / 10;
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let mut grid = input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(move |(x, cost)| ((x, y), cost.to_digit(10).unwrap()))
        })
        .collect::<Grid>();

    println!("Part 1: {}", dijkstra(&grid));
    expand(&mut grid, 5);
    println!("Part 2: {}", dijkstra(&grid));
}
