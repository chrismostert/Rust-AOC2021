fn parse_input(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn p1(input: &[i32]) -> i32 {
    input
        .iter()
        .map(|&num| (num - statistical::median(input)).abs())
        .sum()
}

fn triangle(dist: i32) -> i32 {
    (dist * (dist + 1)) / 2
}

fn p2(input: &[f32]) -> i32 {
    let mean = statistical::mean(input);
    Ord::min(
        input
            .iter()
            .map(|&num| triangle((num as i32 - mean.floor() as i32).abs()))
            .sum(),
        input
            .iter()
            .map(|&num| triangle((num as i32 - mean.ceil() as i32).abs()))
            .sum(),
    )
}

fn main() {
    let input = parse_input(include_str!("../input.txt"));
    let input_float = input.iter().map(|&x| x as f32).collect::<Vec<f32>>();
    println!("Part 1: {}", p1(&input));
    println!("Part 2: {}", p2(&input_float));
}
