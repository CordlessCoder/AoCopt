fn solution(input: &str) -> i32 {
    input
        .bytes()
        .map(|b| match b {
            b'(' => 1,
            b')' => -1,
            _ => 0,
        })
        .sum()
}

fn main() {
    aoc_util::hook_solution(solution)
}
