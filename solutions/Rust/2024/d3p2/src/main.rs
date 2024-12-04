use winnow::{ascii, combinator, error::InputError, token, Parser};

fn solution(mut input: &str) -> u32 {
    let int = ascii::dec_uint::<_, u32, InputError<_>>;
    let parser = combinator::delimited("mul(", combinator::separated_pair(int, ',', int), ')');
    let disabled = (
        "don't()",
        combinator::alt((
            token::take_until::<_, _, InputError<_>>(0.., "do()"),
            combinator::rest_len.flat_map(|len| token::take(len.saturating_sub(1))),
        )),
        combinator::opt("do()"),
    );
    let mut parser = combinator::preceded(combinator::opt(disabled), parser);
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
