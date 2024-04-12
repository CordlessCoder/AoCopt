use aoc_util::hook_solution;
use winnow::{ascii, combinator as combo, error::ContextError, prelude::*, token};

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
enum Color {
    Red = 0,
    Green,
    Blue,
}

fn solve(mut input: &str) -> u32 {
    let lit = |text: &'static str| token::literal::<_, _, ContextError>(text);
    let mut game = combo::delimited("Game ", ascii::dec_uint::<_, u32, ContextError>, ": ");
    let color = combo::alt((
        "red".value(Color::Red),
        "green".value(Color::Green),
        "blue".value(Color::Blue),
    ));
    let mut cube = combo::separated_pair(ascii::dec_uint::<_, u32, ContextError>, ' ', color);
    let mut sum = 0;
    while let Ok(_gameid) = game.parse_next(&mut input) {
        let mut required = [0u32; 3];
        loop {
            let mut cubes = [0u32; 3];
            // Handle all the sets
            while let Ok((count, color)) = cube.parse_next(&mut input) {
                cubes[color as usize] += count;
                if lit(", ").parse_next(&mut input).is_err() {
                    break;
                }
            }
            // End of set
            cubes
                .into_iter()
                .enumerate()
                .for_each(|(idx, count)| required[idx] = required[idx].max(count));
            if lit("; ").parse_next(&mut input).is_err() {
                // End of game
                break;
            }
        }
        let power: u32 = required.into_iter().product();
        sum += power;
        // Consume newline
        _ = lit("\n").parse_next(&mut input);
    }
    sum
}

fn main() {
    hook_solution(solve)
}
