use winnow::{ascii, combinator, error::InputError, token, Parser};

fn solution(mut input: &str) -> u32 {
    let int = ascii::dec_uint::<_, u32, InputError<_>>;
    let mut parser = combinator::delimited("mul(", combinator::separated_pair(int, ',', int), ')');
    let mut sum = 0;
    while !input.is_empty() {
        if token::take_until::<_, _, InputError<_>>(0.., "mul(")
            .parse_next(&mut input)
            .is_err()
        {
            break;
        }
        let Ok((a, b)) = parser.parse_next(&mut input) else {
            continue;
        };
        sum += a * b;
    }
    sum
}

fn main() {
    aoc_util::hook_solution(solution);
}
