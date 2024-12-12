// 2333133121414131402
// 00...111...2...333.44.5555.6666.777.888899
fn solution(input: &str) -> u64 {
    let input = input.trim().as_bytes();
    // let [mut left, mut right] = [None; 2];
    let mut blocks = input
        .iter()
        .copied()
        .map(|b| b.wrapping_sub(b'0'))
        .enumerate()
        .flat_map(|(i, count)| {
            let block = (i % 2 == 0).then_some(i as u64 / 2);
            std::iter::repeat(block).take(count as usize)
        });
    let compacted = std::iter::from_fn(|| match blocks.next()? {
        Some(block) => Some(block),
        None => blocks.by_ref().rev().find_map(|b| b),
    });
    compacted.enumerate().map(|(i, b)| i as u64 * b).sum()
}

fn main() {
    aoc_util::hook_solution(solution);
}
