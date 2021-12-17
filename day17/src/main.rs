type Xrange = (usize, usize);
type Yrange = (isize, isize);
type Coord = (usize, isize);

fn trajectory(x: usize, y: isize) -> impl Iterator<Item = Coord> {
    (0..).scan((0, 0), move |(xres, yres), t| {
        *xres += x.saturating_sub(t);
        *yres += y - t as isize;
        Some((*xres, *yres))
    })
}

fn hit((x, y): Coord, (xs, xe): Xrange, (ys, ye): Yrange) -> bool {
    (xs..=xe).contains(&x) && (ys..=ye).contains(&y)
}

fn get_trajectories(target_xrange: Xrange, target_yrange: Yrange) -> (isize, usize) {
    let mut max_y = 0;
    let mut amount = 0;
    for x in 0..=target_xrange.1 {
        'yloop: for y in target_yrange.0..=target_yrange.0.abs() {
            let mut max = isize::MIN;
            for c in
                trajectory(x, y).take_while(|(a, b)| a <= &target_xrange.1 && b >= &target_yrange.0)
            {
                if c.1 > max {
                    max = c.1;
                }
                if hit(c, target_xrange, target_yrange) {
                    amount += 1;
                    if max > max_y {
                        max_y = max;
                    }
                    continue 'yloop;
                }
            }
        }
    }
    (max_y, amount)
}

fn main() {
    let (max_y, amount) = get_trajectories((124, 174), (-123, -86));
    println!("Part 1: {}", max_y);
    println!("Part 2: {}", amount);
}
