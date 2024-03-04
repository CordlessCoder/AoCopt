use aoc_util::*;
use winnow::{ascii, combinator as combo, PResult, Parser};

fn parse_number(input: &mut &str) -> PResult<u32> {
    combo::preceded(ascii::multispace0, ascii::dec_uint).parse_next(input)
}

fn parse_card_header(input: &mut &str) -> PResult<u32> {
    combo::delimited("Card ", parse_number, ": ").parse_next(input)
}

fn solve(mut input: &str) -> u32 {
    let mut sum = 0;
    while !input.is_empty() {
        let mut winning = [false; 100];
        if parse_card_header(&mut input).is_err() {
            break;
        }
        let mut numbers = combo::iterator(input, parse_number);
        numbers.for_each(|n| winning[n as usize] = true);
        (input, _) = numbers.finish().unwrap();
        (ascii::multispace0::<_, ()>, '|')
            .parse_next(&mut input)
            .unwrap();
        let mut numbers = combo::iterator(input, parse_number);
        let winning_count = numbers.filter(|&n| winning[n as usize]).count() as u32;
        sum += if winning_count == 0 {
            0
        } else {
            1 << (winning_count - 1)
        };
        (input, _) = numbers.finish().unwrap();
        _ = ascii::line_ending::<_, ()>(&mut input);
    }
    sum
}

fn main() {
    hook_solution(solve)
}
