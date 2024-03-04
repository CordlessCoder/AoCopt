use winnow::{combinator as combo, prelude::*};

fn solution(input: &str) -> u32 {
    let number = winnow::ascii::dec_uint::<_, u32, ()>;
    let x = 'x';
    let parser = (number, x, number, x, number).map(|(x, _, y, _, z)| [x, y, z]);
    combo::repeat(.., combo::terminated(parser, combo::opt('\n')))
        .fold(
            || 0,
            |sum, mut cube| {
                if cube[0] > cube[1] {
                    cube.swap(0, 1);
                }
                if cube[1] > cube[2] {
                    cube.swap(1, 2);
                }
                if cube[0] > cube[1] {
                    cube.swap(0, 1);
                }

                let around = (cube[0] + cube[1]) * 2;
                let bow: u32 = cube.into_iter().product();
                sum + around + bow
            },
        )
        .parse(input)
        .unwrap()
}

fn main() {
    aoc_util::hook_solution(solution)
}
