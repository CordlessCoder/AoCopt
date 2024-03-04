fn solution(input: &str) -> usize {
    input
        .bytes()
        .map(|b| match b {
            b'(' => 1,
            b')' => -1,
            _ => 0,
        })
        .scan(0, |acc, m| {
            *acc += m;
            Some(*acc)
        })
        .position(|b| b == -1)
        .map(|pos| pos + 1)
        .expect("Santa doesn't enter the first floor in the given input.")
}

fn main() {
    aoc_util::hook_solution(solution)
}
