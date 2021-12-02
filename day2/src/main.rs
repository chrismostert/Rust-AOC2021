fn main() {
    let input = include_str!("../input.txt");
    let (pos_x, depth, depth_aim, _) =
        input
            .lines()
            .fold((0, 0, 0, 0), |(pos_x, depth, depth_aim, aim), direction| {
                let splits = direction.split_once(" ").expect("Invalid instruction");
                match (splits.0, splits.1.parse::<i32>().unwrap()) {
                    ("forward", val) => (pos_x + val, depth, depth_aim + aim * val, aim),
                    ("up", val) => (pos_x, depth - val, depth_aim, aim - val),
                    ("down", val) => (pos_x, depth + val, depth_aim, aim + val),
                    _ => unreachable!(),
                }
            });

    println!("Part 1: {}", pos_x * depth);
    println!("Part 2: {}", pos_x * depth_aim);
}
