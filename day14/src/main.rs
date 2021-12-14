use itertools::Itertools;
use std::collections::HashMap;

fn get_rules(input: &str) -> HashMap<(char, char), char> {
    input
        .lines()
        .filter_map(|x| x.split_once(" -> "))
        .map(|(pair, insertion)| {
            (
                (pair.chars().next().unwrap(), pair.chars().nth(1).unwrap()),
                insertion.chars().next().unwrap(),
            )
        })
        .collect::<HashMap<_, _>>()
}

fn do_step(
    pair_counts: &mut HashMap<(char, char), usize>,
    char_counts: &mut HashMap<char, usize>,
    rules: &HashMap<(char, char), char>,
) {
    let old_counts = pair_counts.clone();
    for ((a, b), n) in old_counts {
        if let Some(&insertion) = rules.get(&(a, b)) {
            *pair_counts.entry((a, b)).or_insert(0) -= n;
            *pair_counts.entry((a, insertion)).or_insert(0) += n;
            *pair_counts.entry((insertion, b)).or_insert(0) += n;
            *char_counts.entry(insertion).or_insert(0) += n;
        }
    }
}

fn main() {
    let (polymer_input, rules_input) = include_str!("../input.txt").split_once("\n\n").unwrap();
    let mut pair_counts = polymer_input
        .chars()
        .tuple_windows::<(_, _)>()
        .counts();
    let mut char_counts = polymer_input.chars().counts();
    let rules = get_rules(rules_input);
    let ans = |c: &HashMap<_, _>| c.values().max().unwrap() - c.values().min().unwrap();

    for i in 0..40 {
        do_step(&mut pair_counts, &mut char_counts, &rules);
        if i == 9 {
            println!("Part 1: {}", ans(&char_counts));
        }
    }
    println!("Part 2: {}", ans(&char_counts));
}
