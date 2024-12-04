fn solution(input: &str) -> usize {
    let width = input.bytes().take_while(|&b| b != b'\n').count();
    let get = |x: isize, y: isize| {
        let x: usize = x.try_into().ok()?;
        let y: usize = y.try_into().ok()?;
        input.as_bytes().get(y * (width + 1) + x).copied()
    };
    (0..input.len().div_ceil(width) as isize)
        .flat_map(|y| (0..width as isize).map(move |x| (x, y)))
        // Filter for the middle A
        .filter(|&(x, y)| get(x, y) == Some(b'A'))
        // Filter for \ diagonal
        .filter(|&(x, y)| {
            let Some(top_left) = get(x - 1, y - 1) else {
                return false;
            };
            let Some(bottom_right) = get(x + 1, y + 1) else {
                return false;
            };
            matches!((top_left, bottom_right), (b'M', b'S') | (b'S', b'M'))
        })
        // Filter for / diagonal
        .filter(|&(x, y)| {
            let Some(top_right) = get(x + 1, y - 1) else {
                return false;
            };
            let Some(bottom_left) = get(x - 1, y + 1) else {
                return false;
            };
            matches!((top_right, bottom_left), (b'M', b'S') | (b'S', b'M'))
        })
        .count()
}

fn main() {
    aoc_util::hook_solution(solution);
}
