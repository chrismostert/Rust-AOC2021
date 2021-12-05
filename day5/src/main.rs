type Line = ((usize, usize), (usize, usize));
const WIDTH: usize = 1000;

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

fn get_points(line: &Line, diagonal: bool) -> Vec<(usize, usize)> {
    let mut res = vec![];
    let ((a_x, a_y), (b_x, b_y)) = *line;

    if is_straight(line) {
        for i in if a_x <= b_x {a_x..b_x+1} else {b_x..a_x+1} {
            for j in if a_y <= b_y {a_y..b_y+1} else {b_y..a_y+1} {
                res.push((i,j));
            }
        }
    } else if diagonal {
        let xrange: Vec<usize> = if a_x <= b_x {(a_x..b_x+1).collect()} else {(b_x..a_x+1).rev().collect()};
        let yrange: Vec<usize> = if a_y <= b_y {(a_y..b_y+1).collect()} else {(b_y..a_y+1).rev().collect()};

        for (&x,&y) in xrange.iter().zip(yrange.iter()) {
            res.push((x,y));
        }
    }

    res
}

fn get_answer(input: &[Line], diagonal: bool) -> usize {
    let points = input
            .iter()
            .map(|line| get_points(line, diagonal));
    
    let mut grid = vec![vec![0; WIDTH]; WIDTH];
    
    for point in points.flatten() {
        grid[point.0][point.1] += 1;
    }
    grid.iter().flatten().filter(|&num| *num >= 2).count()
}


fn main() {
    let input: Vec<Line> = include_str!("../input.txt")
        .lines()
        .map(|line| parse_line(line))
        .collect();

    println!("Part 1: {}", get_answer(&input, false));
    println!("Part 2: {}", get_answer(&input, true));     
}
