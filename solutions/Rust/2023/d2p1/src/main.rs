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
    while let Ok(gameid) = game.parse_next(&mut input) {
        let mut impossible = false;
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
            impossible |= [12, 13, 14]
                .into_iter()
                .enumerate()
                .map(|(idx, limit)| (cubes[idx], limit))
                .any(|(count, limit)| count > limit);
            if lit("; ").parse_next(&mut input).is_err() {
                // End of game
                break;
            }
        }
        if !impossible {
            sum += gameid;
        }
        // Consume newline
        _ = lit("\n").parse_next(&mut input);
    }
    sum
}

fn main() {
    hook_solution(solve)
}
