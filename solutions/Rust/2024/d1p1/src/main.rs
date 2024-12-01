use aoc_util::hook_solution;
use winnow::{ascii, combinator, error::InputError};

fn solution(input: &str) -> u32 {
    let int = ascii::dec_uint::<_, u32, InputError<_>>;
    let pair = combinator::separated_pair(int, ascii::space1, int);
    let line = combinator::preceded(ascii::multispace0, pair);
    let (mut left, mut right): (Vec<u32>, Vec<u32>) = combinator::iterator(input, line).unzip();
    left.sort_unstable();
    right.sort_unstable();
    left.into_iter()
        .zip(right)
        .map(|(a, b)| a.abs_diff(b))
        .sum()
}

fn main() {
    hook_solution(solution);
}
