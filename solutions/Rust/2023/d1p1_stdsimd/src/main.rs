#![feature(portable_simd)]
use std::simd::{self, prelude::*};

type Vector = simd::Simd<u8, 16>;
type Mask = simd::Mask<i8, 16>;

pub fn solution(text: &str) -> u32 {
    // unsafe {
    let bytes = text.as_bytes();
    let (prefix, suffix);
    let chunks: &[Vector];
    (prefix, chunks, suffix) = bytes.as_simd();
    // Process the prefix
    let (mut first, mut last, mut state) = solve_iter(prefix, None);
    let [mut first_vector, mut last_vector] = [Vector::splat(0); 2];
    // Vectorized section
    let mut counter = 0;
    for chunk in chunks {
        let [mut first_digit_mask, mut last_digit_mask] = [Mask::splat(false); 2];
        let mut newlines = chunk.simd_eq(Vector::splat(b'\n'));
        let digits = chunk - Vector::splat(b'0');
        let mut digit_mask = digits.simd_le(Vector::splat(9));
        while digit_mask.any() {
            let Some(newline) = newlines.first_set() else {
                // TODO:
                // This chunk isn't terminated by a newline. Consume all digits and save the last
                // one
                // SAFETY: We know there is at least one bit set in the mask because of
                // digit_mask.any()
                let (first_digit, last_digit) =
                    unsafe { get_first_and_last_set(digit_mask).unwrap_unchecked() };
                if state.is_none() {
                    first_digit_mask.set(first_digit as usize, true);
                }
                state = Some(digits[last_digit as usize]);
                break;
            };
            // Consume the newline
            newlines.set(newline, false);
            // Find all digits on this line
            let digits_on_this_line = digit_mask
                & Mask::from_bitmask((u64::MAX >> (64 - Vector::LEN)) >> (Vector::LEN - newline));
            let Some((first_digit, last_digit)) = get_first_and_last_set(digits_on_this_line)
            else {
                // This line contains no digits and is terminated by a newline, just ignore it
                // TODO:
                // Apply the old state
                last += state.take().unwrap_or(0) as u32;
                continue;
            };
            digit_mask ^= digits_on_this_line;

            if state.take().is_none() {
                first_digit_mask.set(first_digit as usize, true)
            }
            last_digit_mask.set(last_digit as usize, true);
        }
        if newlines.any() {
            last += state.take().unwrap_or(0) as u32;
        }
        let first_digits = first_digit_mask.select(digits, Vector::splat(0));
        first_vector += first_digits;
        let last_digits = last_digit_mask.select(digits, Vector::splat(0));
        last_vector += last_digits;
        counter += 1;
        if counter >= (u8::MAX / 9) {
            first += first_vector
                .as_array()
                .iter()
                .map(|&n| n as u32)
                .sum::<u32>();
            last += last_vector
                .as_array()
                .iter()
                .map(|&n| n as u32)
                .sum::<u32>();
            [first_vector, last_vector] = [Vector::splat(0); 2];
            counter = 0;
        }
    }
    // dbg!(first_vector, last_vector);

    // Process the suffix
    first += first_vector
        .as_array()
        .iter()
        .map(|&n| n as u32)
        .sum::<u32>();
    last += last_vector
        .as_array()
        .iter()
        .map(|&n| n as u32)
        .sum::<u32>();
    let (suffix_first, suffix_last, state) = solve_iter(suffix, state);
    let suffix_last = suffix_last + state.unwrap_or(0) as u32;
    first += suffix_first;
    last += suffix_last;
    first * 10 + last
}

fn get_first_and_last_set(mask: Mask) -> Option<(u32, u32)> {
    if !mask.any() {
        return None;
    }
    let mask = mask.to_bitmask();
    let first_idx = mask.trailing_zeros();
    let last_idx = 64 - mask.leading_zeros() - 1;
    Some((first_idx, last_idx))
}

pub fn solve_iter(input: &[u8], mut state: Option<u8>) -> (u32, u32, Option<u8>) {
    let [mut first, mut last] = [0; 2];
    for &byte in input {
        if byte == b'\n' {
            last += state.take().unwrap_or(0) as u32;
            continue;
        }
        let num = byte.wrapping_sub(b'0');
        if num > 9 {
            // This isn't a digit
            continue;
        }
        if state.is_none() {
            first += num as u32;
        }
        state = Some(num);
    }
    (first, last, state)
}

fn main() {
    aoc_util::hook_solution(solution)
}
