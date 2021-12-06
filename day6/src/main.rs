fn read_input(input: &str) -> Vec<usize> {
    let mut res = vec![0; 9];

    for num in input
        .split(',')
        .map(|numstr| numstr.parse::<usize>().unwrap())
    {
        res[num] += 1;
    }

    res
}

fn update(mut counts: Vec<usize>) -> Vec<usize> {
    let reproducing = counts.drain(0..1).next().unwrap();
    counts[6] += reproducing;
    counts.push(reproducing);
    counts
}

fn simulate(inputs: &Vec<usize>, times: usize) -> usize {
    let mut res = inputs.clone();
    for _ in 0..times {
        res = update(res);
    }

    res.iter().sum()
}

fn main() {
    let input = read_input(include_str!("../input.txt"));

    println!("Part 1: {}", simulate(&input, 80));
    println!("Part 2: {}", simulate(&input, 256))
}
