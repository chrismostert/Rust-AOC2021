use std::collections::HashMap;

type Line = ((usize, usize), (usize, usize));

fn parse_line(input: &str) -> Line {
    let (a, b) = input.split_once(" -> ").unwrap();
    let (a_x, a_y) = a.split_once(',').unwrap();
    let (b_x, b_y) = b.split_once(',').unwrap();

    (
        (a_x.parse().unwrap(), a_y.parse().unwrap()),
        (b_x.parse().unwrap(), b_y.parse().unwrap()),
    )
}

fn is_straight(line: &Line) -> bool {
    let ((a_x, a_y), (b_x, b_y)) = line;
    (a_x == b_x) | (a_y == b_y)
}

fn range(start: usize, end: usize) -> Box<dyn Iterator<Item = usize>> {
    match start.cmp(&end) {
        std::cmp::Ordering::Equal => Box::new(std::iter::repeat(start)),
        std::cmp::Ordering::Greater => Box::new((end..start + 1).rev()),
        std::cmp::Ordering::Less => Box::new(start..end + 1),
    }
}

fn get_points(line: &Line, diagonal: bool) -> Vec<(usize, usize)> {
    let mut res = vec![];
    let ((a_x, a_y), (b_x, b_y)) = *line;

    for (x, y) in range(a_x, b_x).zip(range(a_y, b_y)) {
        if is_straight(line) | diagonal {
            res.push((x, y));
        }
    }

    res
}

fn get_answer(input: &[Line], diagonal: bool) -> usize {
    let points = input.iter().map(|line| get_points(line, diagonal));
    let mut grid: HashMap<(usize, usize), i32> = HashMap::new();

    for point in points.flatten() {
        *grid.entry((point.0, point.1)).or_insert(0) += 1;
    }
    
    grid.iter().filter(|(_, &v)| v >= 2).count()
}

fn main() {
    let input: Vec<Line> = include_str!("../input.txt")
        .lines()
        .map(|line| parse_line(line))
        .collect();

    println!("Part 1: {}", get_answer(&input, false));
    println!("Part 2: {}", get_answer(&input, true));
}
