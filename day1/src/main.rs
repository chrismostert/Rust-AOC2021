fn main() {
    let input: Vec<u32> = include_str!("../input.txt")
        .lines()
        .map(|line| line.parse::<u32>().expect("Not a number!"))
        .collect();

    let mut n_inc = 0;
    for i in 1..input.len() {
        if input[i] > input[i-1] {
            n_inc += 1;
        }
    }

    println!("Part 1: {}", n_inc);
}
