use itertools::Itertools;
use hashbrown::HashSet;

fn parse_scanners(input: &str) -> Vec<Vec<(isize, isize, isize)>> {
    input.split("\n\n").map(to_coords).collect()
}
fn to_coords(input: &str) -> Vec<(isize, isize, isize)> {
    input
        .lines()
        .enumerate()
        .fold(Vec::default(), |mut acc, (i, x)| {
            if i != 0 {
                let mut spl = x.split(',');
                acc.push((
                    spl.next().unwrap().parse().unwrap(),
                    spl.next().unwrap().parse().unwrap(),
                    spl.next().unwrap().parse().unwrap(),
                ));
            }
            acc
        })
}

fn rotate((x, y, z): (isize, isize, isize), no: usize) -> (isize, isize, isize) {
    match no {
        0 => (x, y, z),
        1 => (x, z, -y),
        2 => (x, -y, -z),
        3 => (x, -z, y),
        4 => (y, x, -z),
        5 => (y, z, x),
        6 => (y, -x, z),
        7 => (y, -z, -x),
        8 => (z, x, y),
        9 => (z, y, -x),
        10 => (z, -x, -y),
        11 => (z, -y, x),
        12 => (-x, y, -z),
        13 => (-x, z, y),
        14 => (-x, -y, z),
        15 => (-x, -z, -y),
        16 => (-y, x, z),
        17 => (-y, z, -x),
        18 => (-y, -x, -z),
        19 => (-y, -z, x),
        20 => (-z, x, -y),
        21 => (-z, y, x),
        22 => (-z, -x, y),
        23 => (-z, -y, -x),
        _ => unreachable!(),
    }
}

fn translate_scanner(
    total: &mut HashSet<(isize, isize, isize)>,
    scanner: &[(isize, isize, isize)],
) -> Option<(isize, isize, isize)> {
    for r in 0..24 {
        // Can be in any orientation
        let rotated = scanner
            .iter()
            .map(|&(x, y, z)| rotate((x, y, z), r))
            .collect::<Vec<_>>();

        // Every point in the scanner could eventually end up in any of the points we've already translated
        let possible_translations = total
            .iter()
            .cartesian_product(&rotated)
            .map(|((x1, y1, z1), (x2, y2, z2))| (x1 - x2, y1 - y2, z1 - z2));

        // If we found a translation that overlaps with more than 12 points, we add it to the total
        for (dx, dy, dz) in possible_translations {
            let translated = rotated.iter().map(|(x, y, z)| (x + dx, y + dy, z + dz));
            if translated.clone().filter(|vec| total.contains(vec)).count() >= 12 {
                total.extend(translated);
                return Some((dx, dy, dz));
            }
        }
    }
    None
}

fn main() {
    let mut scanners = parse_scanners(include_str!("../input.txt"));

    // We take the first scanner as reference
    let mut total: HashSet<(isize, isize, isize)> = scanners.remove(0).into_iter().collect();
    let mut dists = vec![(0, 0, 0)];

    while !scanners.is_empty() {
        for i in (0..scanners.len()).rev() {
            if let Some(d) = translate_scanner(&mut total, &scanners[i]) {
                // Found translation of a new scanner
                dists.push(d);
                scanners.remove(i);
            }
        }
    }

    println!("Part 1: {}", total.len());
    println!(
        "Part 2: {}",
        dists
            .iter()
            .tuple_combinations()
            .map(|((x1, y1, z1), (x2, y2, z2))| {
                (x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs()
            })
            .max()
            .unwrap()
    )
}
