fn p1(input: &[usize]) -> usize {
    let mut gamma = 0;
    let half = input.len() / 2;

    for i in 0..12 {
        if input.iter().filter(|&num| (num & (1 << i)) > 0).count() > half {
            gamma += 1 << i
        }
    }

    gamma * (!gamma & 0xFFF)
}

fn p2(input: &[usize]) -> usize {
    let oxygen = get_reading(input, 11, true);
    let co2 = get_reading(input, 11, false);

    oxygen * co2
}

fn get_reading(input: &[usize], pos: i32, most: bool) -> usize {
    let ones = input.iter().filter(|&num| (num & (1 << pos) > 0)).count();
    let zeros = input.len() - ones;
    let keep: usize;

    if most {
        keep = if ones >= zeros { 1 } else { 0 };
    } else {
        keep = if zeros <= ones { 0 } else { 1 };
    }

    let filtered: Vec<usize> = input
        .iter()
        .filter(|&num| {
            if keep == 1 {
                (num & (1 << pos)) > 0
            } else {
                (num & (1 << pos)) == 0
            }
        })
        .copied()
        .collect();

    if filtered.len() == 1 {
        return filtered[0];
    }

    get_reading(&filtered, pos - 1, most)
}

fn main() {
    let input: Vec<usize> = include_str!("../input.txt")
        .lines()
        .map(|binary| usize::from_str_radix(binary, 2).expect("Not a valid number"))
        .collect();

    println!("Part 1: {}", p1(&input));
    println!("Part 2: {}", p2(&input));
}
