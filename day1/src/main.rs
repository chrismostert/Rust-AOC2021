fn main() {
    let input: Vec<u32> = include_str!("../input.txt")
        .lines()
        .map(|line| line.parse::<u32>().expect("Not a number!"))
        .collect();

    let windows: Vec<u32> = input
        .windows(3)
        .map(|window| window.iter().sum::<u32>())
        .collect();

    fn get_n_inc(input: Vec<u32>) -> u32 {
        let mut res = 0;

        for i in 1..input.len() {
            if input[i] > input[i - 1] {
                res += 1;
            }
        }

        res
    }
    
    println!("Part 1: {}", get_n_inc(input));
    println!("Part 2: {}", get_n_inc(windows));
}
