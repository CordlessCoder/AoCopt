use winnow::{ascii, combinator, error::InputError, Parser};

fn solution(mut input: &str) -> u32 {
    let int = ascii::dec_uint::<_, u32, InputError<_>>;
    let mut parser = combinator::delimited("mul(", combinator::separated_pair(int, ',', int), ')');
    let mut sum = 0;
    while !input.is_empty() {
        let Ok((a, b)) = parser.parse_next(&mut input) else {
            input = &input[1..];
            continue;
        };
        sum += a * b;
    }
    sum
}

fn main() {
    aoc_util::hook_solution(solution);
}
