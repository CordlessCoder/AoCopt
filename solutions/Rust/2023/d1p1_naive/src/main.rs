use aoc_util::*;

fn solution(input: &str) -> u32 {
    let (first, last) = input
        .as_bytes()
        .split(|&b| b == b'\n')
        .map(|line| {
            let mut iter = line
                .iter()
                .copied()
                .map(|b| b.wrapping_sub(b'0'))
                .filter(|&b| b <= 9);
            let first = iter.next().unwrap_or(0);
            let last = iter.next_back().unwrap_or(first);
            (first, last)
        })
        .fold((0, 0), |(first, last), (f, l)| {
            (first + f as u32, last + l as u32)
        });
    first * 10 + last
}

fn main() {
    hook_solution(solution);
}
