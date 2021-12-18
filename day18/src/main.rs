use itertools::Itertools;
type Snail = Vec<(u32, u32)>;

fn to_snail(input: &str) -> Snail {
    let mut res = Vec::default();
    let mut depth = 0;
    for c in input.chars() {
        match c {
            '[' => depth += 1,
            ']' => depth -= 1,
            num if num.is_digit(10) => res.push((num.to_digit(10).unwrap(), depth)),
            _ => (),
        }
    }
    res
}

fn add_list(nums: &[Snail]) -> Snail {
    let mut cur = nums[0].clone();
    for num in nums.iter().skip(1) {
        add(&mut cur, num);
    }
    cur
}

fn add(num: &mut Snail, toadd: &[(u32, u32)]) {
    num.extend(toadd);
    for (_, depth) in num.iter_mut() {
        *depth += 1;
    }
    reduce(num);
}

fn reduce(num: &mut Snail) {
    while explode(num) || split(num) {}
}

fn explode(num: &mut Snail) -> bool {
    for (i, (&(aval, adepth), &(bval, bdepth))) in num.iter().tuple_windows().enumerate() {
        if adepth == 5 && bdepth == 5 {
            if num.get(i.saturating_sub(1)).is_some() && i.saturating_sub(1) != i {
                num.get_mut(i - 1).unwrap().0 += aval;
            }
            if num.get(i + 2).is_some() {
                num.get_mut(i + 2).unwrap().0 += bval;
            }
            num.drain(i..i + 2);
            num.insert(i, (0, 4));
            return true;
        }
    }
    false
}

fn split(num: &mut Snail) -> bool {
    for (i, &(val, depth)) in num.iter().enumerate() {
        if val > 9 {
            num.remove(i);
            num.insert(i, ((((val as f64) / 2f64).floor()) as u32, depth + 1));
            num.insert(i + 1, ((((val as f64) / 2f64).ceil()) as u32, depth + 1));
            return true;
        }
    }
    false
}

fn magnitude(mut num: Snail) -> u32 {
    for d in (1..=4).rev() {
        while magnitude_recurse(&mut num, d) {}
    }
    num[0].0
}

fn magnitude_recurse(num: &mut Snail, d: u32) -> bool {
    for (i, (&(aval, adepth), &(bval, bdepth))) in num.iter().tuple_windows().enumerate() {
        if adepth == d && bdepth == d {
            num[i] = ((3 * aval + 2 * bval), adepth - 1);
            num.remove(i + 1);
            return true;
        }
    }
    false
}

fn main() {
    let input: Vec<Snail> = include_str!("../input.txt").lines().map(to_snail).collect();
    println!("Part 1: {}", magnitude(add_list(&input)));
    let max_mag = input
        .into_iter()
        .permutations(2)
        .map(|perm| magnitude(add_list(&perm)))
        .max()
        .unwrap();
    println!("Part 2: {}", &max_mag);
}
