fn solution(input: &str) -> usize {
    let width = input.bytes().take_while(|&b| b != b'\n').count();
    let get = |x: isize, y: isize| {
        let x: usize = x.try_into().ok()?;
        let y: usize = y.try_into().ok()?;
        input.as_bytes().get(y * (width + 1) + x).copied()
    };
    (0..input.len().div_ceil(width) as isize)
        .flat_map(|y| (0..width as isize).map(move |x| (x, y)))
        .map(|(x, y)| {
            let count_matches = |step_x: isize, step_y: isize| {
                let mut x = x;
                let mut y = y;
                std::iter::from_fn(move || {
                    let ret = get(x, y);
                    x += step_x;
                    y += step_y;
                    ret
                })
                .take(4)
                .eq("XMAS".bytes())
            };
            let offsets = [
                (1, -1),
                (1, 0),
                (1, 1),
                (0, 1),
                (0, -1),
                (-1, 1),
                (-1, 0),
                (-1, -1),
            ];
            offsets
                .into_iter()
                .filter(|&(x, y)| count_matches(x, y))
                .count()
        })
        .sum()
}

fn main() {
    aoc_util::hook_solution(solution);
}
