use std::collections::HashMap;
use std::str;

fn get_slices(input: &str, size: usize) -> impl Iterator<Item = &str> {
    input
        .as_bytes()
        .windows(size)
        .map(|x| str::from_utf8(x).unwrap())
}

fn get_counts<'a>(slices: impl Iterator<Item = &'a str>) -> HashMap<String, usize> {
    slices.fold(HashMap::default(), |mut acc, x| {
        *acc.entry(x.to_owned()).or_insert(0) += 1;
        acc
    })
}

fn get_rules(input: &str) -> HashMap<&str, &str> {
    input.lines().map(|x| x.split_once(" -> ").unwrap()).fold(
        HashMap::default(),
        |mut acc, (pair, insertion)| {
            acc.insert(pair, insertion);
            acc
        },
    )
}

fn do_step(
    pair_counts: &mut HashMap<String, usize>,
    char_counts: &mut HashMap<String, usize>,
    rules: &HashMap<&str, &str>,
) {
    let cur_pairs = pair_counts.clone();
    for (pair, amount) in cur_pairs {
        if let Some(&to_insert) = rules.get(&*pair) {
            match pair_counts[&pair].saturating_sub(amount) {
                0 => {
                    pair_counts.remove(&pair);
                }
                amount => {
                    *pair_counts.get_mut(&pair).unwrap() = amount;
                }
            }

            for to_inc in [
                format!("{}{}", &pair[..1], to_insert),
                format!("{}{}", to_insert, &pair[1..]),
            ] {
                *pair_counts.entry(to_inc).or_insert(0) += amount;
            }

            *char_counts.entry(to_insert.to_owned()).or_insert(0) += amount;
        }
    }
}

fn get_ans(char_counts: &HashMap<String, usize>) -> usize {
    char_counts.values().max().unwrap() - char_counts.values().min().unwrap()
}

fn main() {
    let (polymer_input, rules_input) = include_str!("../input.txt").split_once("\n\n").unwrap();

    let mut pair_counts = get_counts(get_slices(polymer_input, 2));
    let mut char_counts = get_counts(get_slices(polymer_input, 1));
    let rules = get_rules(rules_input);

    for i in 0..40 {
        do_step(&mut pair_counts, &mut char_counts, &rules);
        if i == 9 {
            println!("Part 1: {}", get_ans(&char_counts));
        }
    }
    println!("Part 2: {}", get_ans(&char_counts));
}
