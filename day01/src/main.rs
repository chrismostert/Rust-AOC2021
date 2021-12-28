fn main() {
    let input: Vec<u32> = include_str!("../input.txt")
        .lines()
        .map(|line| line.parse::<u32>().expect("Not a number!"))
        .collect();

    fn get_increments(input: &[u32], window_size: usize) -> usize {
        input
            .iter()
            .zip(input.iter().skip(window_size))
            .filter(|(a, b)| b > a)
            .count()
    }

    println!("Part 1: {}", get_increments(&input, 1));
    println!("Part 2: {}", get_increments(&input, 3));
}
