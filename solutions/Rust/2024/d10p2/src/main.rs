fn solution(input: &str) -> u32 {
    fn score(
        get: impl Fn(usize, usize) -> Option<u8> + Clone,
        x: usize,
        y: usize,
        step: u8,
        goal: u8,
    ) -> u32 {
        let offsets = [
            (x.wrapping_sub(1), y),
            (x, y.wrapping_sub(1)),
            (x.wrapping_add(1), y),
            (x, y.wrapping_add(1)),
        ];
        let next_steps = offsets
            .iter()
            .copied()
            .flat_map(|(x, y)| get(x, y).map(|b| (x, y, b)))
            .filter_map(|(x, y, b)| (b == step).then_some((x, y)));
        if step == goal {
            return next_steps.count() as u32;
        }
        next_steps
            .map(|(x, y)| score(get.clone(), x, y, step + 1, goal))
            .sum()
    }
    let input = input.trim().as_bytes();
    let width = input.iter().position(|&b| b == b'\n').unwrap();
    let height = input.len().div_ceil(width + 1);
    let get = |x: usize, y: usize| {
        if x >= width || y >= height {
            return None;
        };
        Some(input[y * (width + 1) + x])
    };
    input
        .split(|&b| b == b'\n')
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .copied()
                .enumerate()
                .map(move |(x, b)| (x, y, b))
        })
        .filter_map(|(x, y, b)| (b == b'0').then_some((x, y)))
        .map(|(x, y)| score(get, x, y, b'1', b'9'))
        .sum()
}

fn main() {
    aoc_util::hook_solution(solution);
}
