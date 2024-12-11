use winnow::{ascii, combinator, error::InputError};

type Int = u64;

fn solution(input: &str) -> Int {
    fn total_possible(value: Int, target: Int, values: &[Int]) -> bool {
        match values {
            [] => value == target,
            [next, rest @ ..] => {
                total_possible(value + next, target, rest)
                    || total_possible(value * next, target, rest)
                    || total_possible(value * (10u64.pow(next.ilog10() + 1)) + next, target, rest)
            }
        }
    }
    let int = ascii::dec_uint::<&str, Int, InputError<&str>>;
    let rhs =
        combinator::repeat::<_, _, Vec<_>, _, _>(.., combinator::preceded(ascii::space0, int));
    let line = combinator::separated_pair(int, ": ", rhs);
    let line = combinator::terminated(line, ascii::multispace0);

    let mut sum = 0;
    for (total, values) in &mut combinator::iterator(input, line) {
        let Some((&first, rest)) = values.split_first() else {
            continue;
        };
        if total_possible(first, total, rest) {
            sum += total;
        }
    }

    sum
}

fn main() {
    aoc_util::hook_solution(solution);
}
