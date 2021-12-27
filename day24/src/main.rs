// After some analysis we can see that Z is the only state which carries over from input to input.
// Also, we can essentially see Z as a stack to which numbers are pushed and popped (in base 26).
// (A push is times 26, a pop a division by 26, i.e. adding or removing a single digit in base 26)
// There are seven guaranteed pushes (the condition in the middle of the block is impossible to satisfy).
// Thus, to end up with a 0 in Z we need to make sure that the 7 blocks which pop (/26, check is always negative)
// don't push something back.
// This is only possible when the 'eql x w' condition is met, which we can use to deduce the correct digits.
fn solve(input: &str) -> (usize, usize) {
    let (mut max_no, mut min_no) = ([0i8; 14], [0i8; 14]);
    let blocks = input
        .split("inp w")
        .skip(1)
        .map(|block| {
            block
                .lines()
                .enumerate()
                .fold((0, 0), |acc, (pc, instruction)| match pc {
                    5 => (
                        instruction.split(' ').nth(2).unwrap().parse().unwrap(),
                        acc.1,
                    ),
                    15 => (
                        acc.0,
                        instruction.split(' ').nth(2).unwrap().parse().unwrap(),
                    ),
                    _ => acc,
                })
        })
        .collect::<Vec<_>>();

    let mut stack = Vec::default();
    for (block_no, &(check, offset)) in blocks.iter().enumerate() {
        match check >= 0 {
            true => stack.push((block_no, offset)),
            false => {
                let (popped_no, popped_offset) = stack.pop().unwrap();
                let val = popped_offset + check;
                let positive = val >= 0;
                max_no[block_no] = 9 + if positive { 0 } else { val };
                max_no[popped_no] = 9 - if positive { val } else { 0 };
                min_no[block_no] = 1 + if positive { val } else { 0 };
                min_no[popped_no] = 1 - if positive { 0 } else { val };
            }
        }
    }

    let to_num = |numarr: [i8; 14]| {
        numarr.iter().enumerate().fold(0, |acc, (i, &x)| {
            acc + x as usize * 10usize.pow((13 - i) as u32)
        })
    };
    (to_num(max_no), to_num(min_no))
}

fn main() {
    let (max_no, min_no) = solve(include_str!("../input.txt"));
    println!("Part 1: {}", max_no);
    println!("Part 2: {}", min_no);
}
