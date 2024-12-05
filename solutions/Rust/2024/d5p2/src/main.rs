use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use winnow::{ascii, combinator, error as werror, Parser};

fn solution(mut input: &str) -> u32 {
    let int = ascii::dec_uint::<_, u32, werror::InputError<_>>;

    let rule = combinator::separated_pair(int, '|', int);
    let rule = combinator::preceded(ascii::multispace0, rule);
    let mut iter = combinator::iterator(input, rule);
    let rules = (&mut iter).fold(HashSet::with_capacity(128), |mut set, (before, after)| {
        set.insert((before, after));
        set
    });
    input = iter.finish().unwrap().0;

    let mut sum = 0;
    'pages: while !input.is_empty() {
        let rule = combinator::preceded(combinator::opt(','), int);
        let rule = combinator::repeat(.., rule);
        let mut rule = combinator::preceded(ascii::multispace0::<_, werror::InputError<_>>, rule);
        let mut pages: Vec<_> = rule.parse_next(&mut input).unwrap();

        if pages.is_empty() {
            continue;
        }
        'check: {
            let pages = pages.iter().copied().enumerate().fold(
                HashMap::with_capacity(128),
                |mut map, (index, page)| {
                    map.insert(page, index);
                    map
                },
            );
            for (before, after) in &rules {
                let Some(&x) = pages.get(before) else {
                    continue;
                };
                let Some(&y) = pages.get(after) else {
                    continue;
                };
                if x >= y {
                    break 'check;
                }
            }

            continue 'pages;
        }

        {
            let cmp = |a, b| {
                if rules.contains(&(a, b)) {
                    return Ordering::Less;
                }
                if rules.contains(&(b, a)) {
                    return Ordering::Greater;
                }
                Ordering::Equal
            };
            pages.sort_by(|&a, &b| cmp(a, b));
        }

        let mid = pages.len() / 2;
        let mid = pages[mid];
        sum += mid;
    }
    sum
}

fn main() {
    aoc_util::hook_solution(solution)
}
