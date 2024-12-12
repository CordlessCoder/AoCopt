fn solution(input: &str) -> u64 {
    let input = input.trim().as_bytes();
    let blocks = input
        .iter()
        .copied()
        .map(|b| b.wrapping_sub(b'0'))
        .enumerate()
        .map(|(i, count)| {
            let block = (i % 2 == 0).then_some(i as u64 / 2);
            (count, block)
        });
    let mut blocks: Vec<_> = blocks.collect();
    if blocks.is_empty() {
        return 0;
    }
    let mut read = blocks.len() - 1;
    while read > 0 {
        let (len, Some(block)) = blocks[read] else {
            read -= 1;
            continue;
        };
        let Some(free_idx) = blocks
            .iter()
            .take(read)
            .position(|&(space, block)| space >= len && block.is_none())
        else {
            read -= 1;
            continue;
        };
        let free_block = blocks[free_idx];
        let overflow_space = free_block.0 - len;
        // Free up old block
        blocks[read] = (len, None);
        read -= 1;
        // Write the data into the free block
        blocks[free_idx] = (len, Some(block));
        if overflow_space == 0 {
            continue;
        }
        // If there is already a free block following the space we're about to mark as free, just
        // resize that block
        if let Some((count, None)) = blocks.get_mut(free_idx + 1) {
            *count += overflow_space;
            continue;
        }
        // We need to record the remaining space in the free block
        blocks.insert(free_idx + 1, (overflow_space, None));
        read += 1;
    }
    blocks
        .iter()
        .flat_map(|&(count, block)| std::iter::repeat_n(block, count as usize))
        .enumerate()
        .flat_map(|(idx, block)| block.map(|b| idx as u64 * b))
        .sum()
}

fn main() {
    aoc_util::hook_solution(solution);
}
