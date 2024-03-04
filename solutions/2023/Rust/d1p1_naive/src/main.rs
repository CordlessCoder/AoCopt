use aoc_util::*;

fn solution(input: &str) -> u32 {
    let (first, last) = input
        .as_bytes()
        .split(|&b| b == b'\n')
        .map(|line| {
            let mut iter = line.iter().copied();
            let first = iter.find(|&b| b <= b'9').unwrap_or(0);
            let last = iter.rfind(|&b| b <= b'9').unwrap_or(first);
            ((first & 0xf) as u32, (last & 0xf) as u32)
        })
        .fold((0, 0), |(first, last), (f, l)| (first + f, last + l));
    first * 10 + last
}

fn main() {
    hook_solution(solution);
}
