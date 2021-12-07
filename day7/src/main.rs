fn parse_input(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn p1(input: &[i32]) -> i32 {
    let mut sorted = input.to_owned();
    let l = input.len() / 2;
    sorted.sort_unstable();

    let pos = ((sorted[l] + sorted[l - 1]) / 2i32).abs();
    input.iter().map(|num| (num - pos).abs()).sum()
}

fn main() {
    let input = parse_input(include_str!("../input.txt"));
    println!("Part 1: {}", p1(&input));
}
