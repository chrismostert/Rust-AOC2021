use std::collections::VecDeque;

fn read_input(input: &str) -> VecDeque<usize> {
    let mut res = VecDeque::from_iter([0; 9]);
    for num in input
        .split(',')
        .map(|numstr| numstr.parse::<usize>().unwrap())
    {
        res[num] += 1;
    }
    res
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
