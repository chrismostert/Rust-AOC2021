use std::collections::HashMap;

fn create_graph(input: &str) -> HashMap<&str, Vec<&str>> {
    input.lines().fold(HashMap::default(), |mut acc, x| {
        let (a,b) = x.split_once('-').unwrap();
        if acc.get(a).is_some() {acc.get_mut(a).unwrap().push(b);} else {acc.insert(a, vec![b]);};
        if acc.get(b).is_some() {acc.get_mut(b).unwrap().push(a);} else {acc.insert(b, vec![a]);};  
        acc
    })
}

fn get_no_paths<'a> (graph: &HashMap<&str, Vec<&'a str>>, cur: &'a str, path: &mut Vec<&'a str>, mut visited_twice: bool) -> usize {
    if cur == "end" {
        return 1;
    }
    if cur.chars().all(|c| c.is_lowercase()) && path.contains(&cur) {
        if visited_twice || cur == "start" {
            return 0;
        }
        visited_twice = true;
    }
    path.push(cur);
    let ans = graph[cur].iter().map(|x| get_no_paths(graph, x, path, visited_twice)).sum();
    path.pop();
    ans
}

fn main() {
    let input = create_graph(include_str!("../input.txt"));
    println!("Part 1: {}", get_no_paths(&input, "start", &mut Vec::default(), true));
    println!("Part 2: {}", get_no_paths(&input, "start", &mut Vec::default(), false));
}
