fn solution(input: &str) -> u32 {
    let mut bytes = input.bytes();
    let lines = std::iter::from_fn(|| {
        if bytes.len() == 0 {
            return None;
        };
        let line = bytes.by_ref().take_while(|byte| *byte != b'\n');

        let mut digits = line
            .map(|byte| byte.wrapping_sub(b'0'))
            .filter(|&byte| byte <= 9);

        let first = digits.next().unwrap_or(0);
        let last = digits.last().unwrap_or(first);
        Some((first, last))
    });
    let (sum_first, sum_last) = lines.fold((0, 0), |(sum_first, sum_last), (first, last)| {
        (sum_first + first as u32, sum_last + last as u32)
    });
    sum_first * 10 + sum_last
}

fn main() {
    aoc_util::hook_solution(solution);
}
