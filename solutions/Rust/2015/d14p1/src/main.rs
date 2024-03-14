use winnow::{ascii, combinator as combo, prelude::*, seq, token};

struct Reindeer<'input> {
    name: &'input str,
    speed: u32,
    fly: u32,
    rest: u32,
}

// {name} can fly {speed} km/s for {fly} seconds, but then must rest for {rest} seconds.
fn parse_line<'input>(input: &mut &'input str) -> PResult<Reindeer<'input>> {
    seq!(Reindeer {
        name: token::take_until(.., ' '),
    _: ascii::space0,
        _ : "can fly",
    _: ascii::space0,
    speed: ascii::dec_uint,
    _: ascii::space0,
    _: "km/s for",
    _: ascii::space0,
    fly: ascii::dec_uint,
    _: ascii::space0,
    _: "seconds, but then must rest for",
    _: ascii::space0,
    rest: ascii::dec_uint,
    _: ascii::space0,
    _: "seconds.",
    _: ascii::multispace0
    })
    .parse_next(input)
}

fn solve(input: &str, second: u32) -> u32 {
    combo::iterator(input, parse_line)
        .map(
            move |Reindeer {
                      name: _name,
                      speed,
                      fly,
                      rest,
                  }| {
                let interval = fly + rest;
                let complete_flights = second / interval;
                let remainder = second % interval;
                let remainder = remainder.min(fly);
                (complete_flights * fly + remainder) * speed
            },
        )
        .max()
        .expect("Failed to get any input")
}

fn main() {
    aoc_util::hook_solution(|input| solve(input, 2503))
}
