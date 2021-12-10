use std::collections::HashMap;

enum ParseResult {
    Corrupt(usize),
    Completed(usize),
}

fn check_line(line: &str) -> ParseResult {
    let revmap: HashMap<char, char> =
        HashMap::from_iter([('}', '{'), (']', '['), (')', '('), ('>', '<')]);
    let corrupt_scores: HashMap<char, usize> =
        HashMap::from_iter([('{', 1197), ('[', 57), ('(', 3), ('<', 25137)]);
    let fix_scores: HashMap<char, usize> =
        HashMap::from_iter([('{', 3), ('[', 2), ('(', 1), ('<', 4)]);

    let mut last_opened = Vec::default();
    for c in line.chars() {
        if let Some(&v) = revmap.get(&c) {
            if v != last_opened.pop().unwrap() {
                return ParseResult::Corrupt(*corrupt_scores.get(&v).unwrap());
            }
        } else {
            last_opened.push(c);
        }
    }
    ParseResult::Completed(
        last_opened
            .iter()
            .rev()
            .fold(0, |acc, x| acc * 5 + fix_scores.get(x).unwrap()),
    )
}

fn p1(input: &[ParseResult]) -> usize {
    input.iter().fold(0, |mut acc, x| {
        if let ParseResult::Corrupt(v) = x {
            acc += v;
        }
        acc
    })
}

fn p2(input: &[ParseResult]) -> usize {
    let mut res = input.iter().fold(Vec::new(), |mut acc, x| {
        if let ParseResult::Completed(v) = x {
            acc.push(v);
        }
        acc
    });
    res.sort_unstable();
    *res[res.len() / 2]
}

fn main() {
    let input: Vec<ParseResult> = include_str!("../input.txt")
        .lines()
        .map(check_line)
        .collect();
    println!("Part 1: {}", p1(&input));
    println!("Part 2: {}", p2(&input));
}
