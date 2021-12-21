use cached::proc_macro::cached;
use std::mem::swap;

fn normal_game(mut p_self: usize, mut p_other: usize) -> usize {
    let mut die = (1..=100).cycle();
    let (mut s_self, mut s_other, mut n_rolls) = (0, 0, 0);
    loop {
        if s_other >= 1000 {
            return s_self * n_rolls;
        }
        let throw: usize = (0..3).map(|_| die.next().unwrap() as usize).sum();
        n_rolls += 3;
        p_self = (((p_self + throw) - 1) % 10) + 1;
        s_self += p_self;
        swap(&mut p_self, &mut p_other);
        swap(&mut s_self, &mut s_other);
    }
}

#[cached]
fn quantum_game(s_self: usize, s_other: usize, p_self: usize, p_other: usize) -> (usize, usize) {
    if s_other >= 21 {
        return (0, 1);
    }
    [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)]
        .iter()
        .fold((0, 0), |wins, &(roll, times)| {
            let pos = (((p_self + roll) - 1) % 10) + 1;
            let (o, s) = quantum_game(s_other, s_self + pos, p_other, pos);
            (wins.0 + s * times, wins.1 + o * times)
        })
}

fn main() {
    println!("Part 1: {}", normal_game(1, 5));
    let (a, b) = quantum_game(0, 0, 1, 5);
    println!("Part 2: {}", Ord::max(a, b));
}
