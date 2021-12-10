use std::collections::HashMap;

fn check_corrupt(line: &str) -> usize {
    let decreasing: HashMap<char, char> =
        HashMap::from_iter([('}', '{'), (']', '['), (')', '('), ('>', '<')]);
    let scores: HashMap<char, usize> =
        HashMap::from_iter([('{', 1197), ('[', 57), ('(', 3), ('<', 25137)]);
    let mut last_opened = Vec::default();
    for c in line.chars() {
        if let Some(&v) = decreasing.get(&c) {
            if v != last_opened.pop().unwrap() {
                return *scores.get(&v).unwrap();
            }
        } else {
            last_opened.push(c);
        }
    }
    0
}

fn fix_score(line: &str) -> usize {
    let decreasing: HashMap<char, char> =
        HashMap::from_iter([('}', '{'), (']', '['), (')', '('), ('>', '<')]);
    let scores: HashMap<char, usize> = HashMap::from_iter([('{', 3), ('[', 2), ('(', 1), ('<', 4)]);
    let mut last_opened = Vec::default();
    for c in line.chars() {
        if decreasing.get(&c).is_some() {
            last_opened.pop();
        } else {
            last_opened.push(c);
        }
    }
    last_opened
        .iter()
        .rev()
        .fold(0, |acc, x| acc * 5 + scores.get(x).unwrap())
}

fn p1(input: &str) -> usize {
    input.lines().map(check_corrupt).sum()
}

fn p2(input: &str) -> usize {
    let non_corrupt = input.lines().filter(|&line| check_corrupt(line) == 0);
    let mut scores: Vec<usize> = non_corrupt.map(fix_score).collect();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", p1(input));
    println!("Part 2: {}", p2(input));
}
