type Range = (isize, isize);
type Cube = (Range, Range, Range);
#[derive(Clone)]
struct SignedCube {
    sign: bool,
    cube: Cube,
}

fn parse_input(input: &str) -> Vec<SignedCube> {
    input.lines().fold(Vec::default(), |mut cubes, line| {
        let (sign, ranges) = line.split_once(' ').unwrap();
        let sign = match sign {
            "on" => true,
            "off" => false,
            _ => unreachable!(),
        };
        let ranges: Vec<(isize, isize)> = ranges
            .split(',')
            .map(|r| {
                let (from, to) = r.split('=').nth(1).unwrap().split_once("..").unwrap();
                (from.parse().unwrap(), to.parse().unwrap())
            })
            .collect();
        cubes.push(SignedCube {
            sign,
            cube: (ranges[0], ranges[1], ranges[2]),
        });
        cubes
    })
}

fn intersection(slf: &Cube, other: &Cube) -> Option<Cube> {
    let overlap_size = |self_axis: Range, other_axis: Range| {
        (
            Ord::max(self_axis.0, other_axis.0),
            Ord::min(self_axis.1, other_axis.1),
        )
    };
    let overlaps = |axis: Range| axis.0 <= axis.1;
    let (x_overlap, y_overlap, z_overlap) = (
        overlap_size(slf.0, other.0),
        overlap_size(slf.1, other.1),
        overlap_size(slf.2, other.2),
    );

    if overlaps(x_overlap) && overlaps(y_overlap) && overlaps(z_overlap) {
        return Some((x_overlap, y_overlap, z_overlap));
    }
    None
}

fn get_n_lights(input: &[SignedCube]) -> isize {
    let cubes = input
        .iter()
        .fold(Vec::default(), |mut cubes: Vec<SignedCube>, c| {
            let mut next = Vec::default();
            if c.sign {
                next.push(SignedCube {
                    sign: true,
                    cube: c.cube,
                });
            }

            for c2 in &cubes {
                if let Some(intersection) = intersection(&c.cube, &c2.cube) {
                    next.push(SignedCube {
                        sign: !c2.sign,
                        cube: intersection,
                    });
                }
            }

            cubes.extend(next);
            cubes
        });

    cubes.iter().fold(0, |n_lights, cube| {
        let cube_inner = cube.cube;
        let area = (((cube_inner.0).1 - (cube_inner.0).0) + 1)
            * (((cube_inner.1).1 - (cube_inner.1).0) + 1)
            * (((cube_inner.2).1 - (cube_inner.2).0) + 1);
        if cube.sign {
            return n_lights + area;
        }
        n_lights - area
    })
}

fn main() {
    let input = parse_input(include_str!("../input.txt"));
    let small = |axis: Range| axis.0 >= -50 && axis.1 <= 50;
    let input_small = input
        .iter()
        .filter(|&c| small(c.cube.0) && small(c.cube.1) && small(c.cube.2))
        .cloned()
        .collect::<Vec<_>>();

    println!("Part 1: {}", get_n_lights(&input_small));
    println!("Part 2: {}", get_n_lights(&input));
}
