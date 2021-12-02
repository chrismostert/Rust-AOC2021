fn main() {
    let input = include_str!("../input.txt");
    let (pos_x, depth, aim) =
        input
            .lines()
            .fold((0, 0, 0), |(pos_x, depth, aim), direction| {
                let splits = direction.split_once(" ").expect("Invalid instruction");
                match (splits.0, splits.1.parse::<i32>().unwrap()) {
                    ("forward", val) => (pos_x + val, depth + aim * val, aim),
                    ("up", val) => (pos_x, depth, aim - val),
                    ("down", val) => (pos_x, depth, aim + val),
                    _ => unreachable!(),
                }
            });

    println!("Part 1: {}", pos_x * aim);
    println!("Part 2: {}", pos_x * depth);
}
