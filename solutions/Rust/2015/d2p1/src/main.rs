use winnow::{combinator as combo, prelude::*};

fn solution(input: &str) -> u32 {
    let number = winnow::ascii::dec_uint::<_, u32, ()>;
    let x = 'x';
    let parser = (number, x, number, x, number).map(|(x, _, y, _, z)| [x, y, z]);
    combo::repeat(.., combo::terminated(parser, combo::opt('\n')))
        .fold(
            || 0,
            |sum, cube| {
                let sides = [cube[0] * cube[1], cube[1] * cube[2], cube[2] * cube[0]];
                let area = sides.iter().copied().sum::<u32>() * 2 + sides.iter().min().unwrap();
                sum + area
            },
        )
        .parse(input)
        .unwrap()
}

fn main() {
    aoc_util::hook_solution(solution)
}
