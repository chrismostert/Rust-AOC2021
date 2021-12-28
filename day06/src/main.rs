use std::collections::VecDeque;

fn read_input(input: &str) -> VecDeque<usize> {
    input
        .split(',')
        .fold(VecDeque::from_iter([0; 9]), |mut counts, num| {
            counts[num.parse::<usize>().unwrap()] += 1;
            counts
        })
}

fn simulate(inputs: &VecDeque<usize>, times: usize) -> usize {
    let mut res = inputs.to_owned();
    for _ in 0..times {
        res.rotate_left(1);
        res[6] += res[8];
    }
    res.iter().sum()
}

fn main() {
    let input = read_input(include_str!("../input.txt"));
    println!("Part 1: {}", simulate(&input, 80));
    println!("Part 2: {}", simulate(&input, 256))
}
