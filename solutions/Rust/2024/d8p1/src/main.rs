use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn solution(input: &str) -> usize {
    let input = input.trim();
    let width = input.find('\n').unwrap();
    let height = input.len().div_ceil(width + 1);

    let mut antennas: HashMap<u8, HashSet<(usize, usize)>> = HashMap::new();

    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.as_bytes()
                .iter()
                .copied()
                .enumerate()
                .map(move |(x, b)| (x, y, b))
        })
        .filter(|(_, _, b)| *b != b'.')
        .for_each(|(x, y, b)| {
            antennas.entry(b).or_default().insert((x, y));
        });
    let mut cells = vec![false; width * height];

    macro_rules! index {
        [$x:expr, $y:expr] => {
            ($x >= 0 && $y >= 0 && ($x as usize) < width && ($y as usize) < height)
            .then(|| &mut cells[$y as usize * width + $x as usize])
        }
    }

    antennas.iter().for_each(|(_, antennas)| {
        for (&(ax, ay), &(bx, by)) in antennas.iter().tuple_combinations() {
            let offset = (bx as isize - ax as isize, by as isize - ay as isize);
            index!(ax as isize - offset.0, ay as isize - offset.1).map(|cell| *cell = true);
            index!(bx as isize + offset.0, by as isize + offset.1).map(|cell| *cell = true);
        }
    });
    cells.iter().filter(|&&c| c).count()
}

fn main() {
    aoc_util::hook_solution(solution);
}
