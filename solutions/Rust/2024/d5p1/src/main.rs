use std::collections::HashMap;

use winnow::{ascii, combinator, error as werror, Parser};

fn solution(mut input: &str) -> u32 {
    let int = ascii::dec_uint::<_, u32, werror::InputError<_>>;

    let rule = combinator::separated_pair(int, '|', int);
    let rule = combinator::preceded(ascii::multispace0, rule);
    let rules: Vec<_> = combinator::repeat(.., rule).parse_next(&mut input).unwrap();

    let mut sum = 0;
    'pages: while !input.is_empty() {
        _ = ascii::multispace0::<_, werror::InputError<_>>.parse_next(&mut input);
        let rule = combinator::preceded(combinator::opt(','), int);
        let mut iter = combinator::iterator(input, rule);
        let pages =
            (&mut iter)
                .enumerate()
                .fold(HashMap::with_capacity(128), |mut map, (index, page)| {
                    map.insert(page, index);
                    map
                });
        input = iter.finish().unwrap().0;

        if pages.is_empty() {
            continue;
        }
        for (before, after) in &rules {
            let Some(&x) = pages.get(before) else {
                continue;
            };
            let Some(&y) = pages.get(after) else {
                continue;
            };
            if x >= y {
                continue 'pages;
            }
        }

        let mid = pages.len() / 2;
        let mid = *pages.iter().find(|&(_, &index)| index == mid).unwrap().0;
        sum += mid;
    }
    sum
}

fn main() {
    aoc_util::hook_solution(solution)
}
