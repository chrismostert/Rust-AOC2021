#[derive(Default)]
struct Sub {
    pos_horizontal: i32,
    pos_depth: i32,
    pos_depth_aim: i32,
    aim: i32,
}

impl Sub {
    fn execute_direction(&mut self, direction: &str) {
        let splits = direction.split_once(" ").expect("Invalid instruction");
        match (splits.0, splits.1.parse::<i32>().unwrap()) {
            ("forward", val) => {
                self.pos_horizontal += val;
                self.pos_depth_aim += self.aim * val;
            }
            ("up", val) => {
                self.pos_depth -= val;
                self.aim -= val;
            }
            ("down", val) => {
                self.pos_depth += val;
                self.aim += val;
            }
            _ => unreachable!(),
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let mut sub = Sub::default();

    for direction in input.lines() {
        sub.execute_direction(direction);
    }

    println!("Part 1: {}", sub.pos_horizontal * sub.pos_depth);
    println!("Part 2: {}", sub.pos_horizontal * sub.pos_depth_aim);
}
