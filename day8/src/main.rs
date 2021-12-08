use std::collections::HashSet;

fn decode(input_line: &str) -> usize {
    let (signal, output) = input_line.split_once(" | ").unwrap();
    let (one, four) = get_decode_key(signal);
    let mut res = Vec::with_capacity(4);

    for digit in output.split_whitespace().map(to_hashset) {
        let len = digit.len();
        match len {
            2 => res.push(1),
            3 => res.push(7),
            4 => res.push(4),
            7 => res.push(8),
            5 => {
                if four.intersection(&digit).count() == 2 {
                    res.push(2)
                } else if one.intersection(&digit).count() == 2 {
                    res.push(3)
                } else {
                    res.push(5)
                }
            }
            6 => {
                if one.intersection(&digit).count() == 1 {
                    res.push(6)
                } else if four.intersection(&digit).count() == 4 {
                    res.push(9)
                } else {
                    res.push(0)
                }
            }
            _ => unreachable!(),
        }
    }
    res[0] * 1000 + res[1] * 100 + res[2] * 10 + res[3]
}

fn get_decode_key(signal: &str) -> (HashSet<u8>, HashSet<u8>) {
    let splits: Vec<HashSet<u8>> = signal.split_whitespace().map(to_hashset).collect();
    (
        splits.iter().find(|&x| x.len() == 2).unwrap().to_owned(),
        splits.iter().find(|&x| x.len() == 4).unwrap().to_owned(),
    )
}

fn to_hashset(chars: &str) -> HashSet<u8> {
    HashSet::from_iter(chars.bytes())
}

fn main() {
    let decoded = include_str!("../input.txt")
        .lines()
        .map(decode)
        .collect::<Vec<usize>>();

    println!(
        "Part 1: {}",
        decoded
            .iter()
            .map(|&x| {
                x.to_string()
                    .bytes()
                    .filter(|&x| (x == b'1') | (x == b'4') | (x == b'7') | (x == b'8'))
                    .count()
            })
            .sum::<usize>()
    );
    println!("Part 2: {}", decoded.iter().sum::<usize>());
}
