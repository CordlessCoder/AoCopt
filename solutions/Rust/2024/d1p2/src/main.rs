use std::collections::HashMap;

use aoc_util::hook_solution;
use winnow::{ascii, combinator, error::InputError};

fn solution(input: &str) -> u32 {
    let int = ascii::dec_uint::<_, u32, InputError<_>>;
    let pair = combinator::separated_pair(int, ascii::space1, int);
    let line = combinator::preceded(ascii::multispace0, pair);
    let mut left = Vec::with_capacity(256);
    let mut right_counts = HashMap::with_capacity(1024);
    combinator::iterator(input, line).for_each(|(l, r)| {
        left.push(l);
        *right_counts.entry(r).or_insert(0) += 1;
    });
    left.into_iter()
        .map(|n| {
            let count = right_counts.get(&n).copied().unwrap_or(0);
            n * count
        })
        .sum()
}

fn main() {
    hook_solution(solution);
}
