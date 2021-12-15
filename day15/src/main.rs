use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
type Grid = HashMap<(usize, usize), u32>;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    coordinate: (usize, usize),
    cost: u32,
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

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

fn dijkstra(grid: &Grid, dest: (usize, usize)) -> u32 {
    let mut dist: HashMap<(usize, usize), u32> = grid.keys().map(|&k| (k, u32::MAX)).collect();
    let mut frontier = BinaryHeap::new();

    *dist.get_mut(&(0, 0)).unwrap() = 0;
    frontier.push(State {
        coordinate: (0, 0),
        cost: 0,
    });

    while let Some(State { coordinate, cost }) = frontier.pop() {
        if coordinate == dest {
            return cost;
        }
        if cost > dist[&coordinate] {
            continue;
        }
        for n in neighbors(grid, coordinate) {
            let next = State {
                coordinate: n,
                cost: cost + grid[&n],
            };
            if next.cost < dist[&next.coordinate] {
                frontier.push(next);
                *dist.get_mut(&next.coordinate).unwrap() = next.cost;
            }
        }
    }
    0
}

fn size(grid: &Grid) -> (usize, usize) {
    (
        *grid.keys().map(|(x, _)| x).max().unwrap() + 1,
        *grid.keys().map(|(_, y)| y).max().unwrap() + 1,
    )
}

fn expand(grid: &mut Grid, times: usize) {
    let (w, h) = size(grid);
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

    let (w, h) = size(&grid);
    println!("Part 1: {}", dijkstra(&grid, (w - 1, h - 1)));
    expand(&mut grid, 5);
    let (w, h) = size(&grid);
    println!("Part 2: {}", dijkstra(&grid, (w - 1, h - 1)));
}
