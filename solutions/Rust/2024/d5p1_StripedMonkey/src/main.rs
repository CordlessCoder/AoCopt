use std::cmp::Ordering;

fn solution(input: &str) -> usize {
    let (ordering_rules, page_updates) = parse(input);
    page_updates
        .into_iter()
        .filter_map(|update| {
            for rule in ordering_rules.iter() {
                let first = update.iter().position(|e| *e == rule.0);
                let second = update.iter().position(|e| *e == rule.1);

                if let (Some(first), Some(second)) = (first, second) {
                    if first > second {
                        return None;
                    }
                }
            }
            Some(update[update.len() / 2])
        })
        .sum()
}

pub fn parse(input: &str) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let mut lines = input.lines();
    let mut ordering_rules = Vec::new();
    for line in &mut lines {
        if line.is_empty() {
            break;
        }
        let Some((left, right)) = line.split_once("|") else {
            panic!("Invalid");
        };
        let left = left.parse::<usize>().unwrap();
        let right = right.parse::<usize>().unwrap();
        ordering_rules.push((left, right));
    }
    let page_updates: Vec<Vec<usize>> = lines
        .map(|line| {
            line.split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect()
        })
        .collect();
    (ordering_rules, page_updates)
}

pub fn compare(ordering_rules: &[(usize, usize)]) -> impl FnMut(&usize, &usize) -> Ordering + '_ {
    |a, b| {
        if ordering_rules.contains(&(*a, *b)) {
            Ordering::Greater
        } else if ordering_rules.contains(&(*b, *a)) {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}
fn main() {
    aoc_util::hook_solution(solution);
}
