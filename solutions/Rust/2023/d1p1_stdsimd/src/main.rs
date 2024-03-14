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
    for chunk in chunks {
        let [mut first_digit_mask, mut last_digit_mask] = [Mask::splat(false); 2];
        let mut newlines = chunk.simd_eq(Vector::splat(b'\n'));
        let digits = chunk - Vector::splat(b'0');
        let mut digit_mask = digits.simd_le(Vector::splat(9));
        dbg!(digit_mask);
        dbg!(newlines);
        while digit_mask.any() {
            let Some(newline) = newlines.first_set() else {
                // TODO:
                // This chunk isn't terminated by a newline. Consume all digits and save the last
                // one
                let Some((first_digit, last_digit)) = get_first_and_last_set(digit_mask) else {
                    // This chunk doesn't contain any digits, there's nothing to save
                    break;
                };
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

            if state.is_none() {
                first_digit_mask.set(first_digit as usize, true);
            }
            last_digit_mask.set(last_digit as usize, true);
            // dbg!(first_digit_mask.select(digits, Vector::splat(0)));
            // dbg!(last_digit_mask.select(digits, Vector::splat(0)));
            // todo!()
        }
        let first_digits = first_digit_mask.select(digits, Vector::splat(0));
        first_vector += first_digits;
        let last_digits = first_digit_mask.select(digits, Vector::splat(0));
        last_vector += last_digits;
    }

    //         let zero = _mm256_setzero_si256();
    //
    //         let bytes = text.as_bytes();
    //         let mut chunks_iter = bytes.chunks_exact(32);
    //         let mut chunks = chunks_iter
    //             .by_ref()
    //             .map(|chunk| <&[u8] as TryInto<[u8; 32]>>::try_into(chunk).unwrap_unchecked());
    //
    //         // Digits carried over from the end of the previous chunk, Some(last)
    //         let mut state = None;
    //
    //         // 8 bit sums, needs to be flushed every 28 chunks to prevent overflow
    //         let mut sum_first = zero;
    //         let mut sum_last = zero;
    //
    //         let mut total_first: u32 = 0;
    //         let mut total_last: u32 = 0;
    //
    //         for (iter, chunk) in chunks.by_ref().enumerate() {
    //             // SAFETY: loadu has no alignment requirement
    //             let raw = _mm256_loadu_si256(chunk.as_ptr().cast());
    //
    //             let newlines = _mm256_cmpeq_epi8(raw, _mm256_set1_epi8(b'\n' as i8));
    //             let mut newlines = _mm256_movemask_epi8(newlines) as u32;
    //
    //             let vdigits = _mm256_sub_epi8(raw, _mm256_set1_epi8(b'0' as i8));
    //             let vdigit_mask = _mm256_add_epi8(raw, _mm256_set1_epi8(i8::MAX - b'9' as i8));
    //             let vdigit_mask = _mm256_cmpgt_epi8(vdigit_mask, _mm256_set1_epi8(i8::MAX - 10));
    //             let digit_mask = _mm256_movemask_epi8(vdigit_mask) as u32;
    //
    //             // The bitmasks of first and last digits to be extracted
    //             let mut first_digits = 0;
    //             let mut last_digits = 0;
    //
    //             let mut remaining_digits = digit_mask;
    //
    //             while remaining_digits != 0 {
    //                 // We have more digits to process
    //
    //                 if newlines == 0 {
    //                     // This is a partial line
    //                     // We need to consume every digit on the line
    //
    //                     let (first_mask, _last_mask) = get_first_and_last_bitmask(remaining_digits);
    //                     // SAFETY: Must be < 32 as remaining_digits != 0, therefore some bit has to
    //                     // be set
    //                     let last_idx = 31 - remaining_digits.leading_zeros();
    //                     if state.is_none() {
    //                         // If we haven't saved the first digit yet
    //                         first_digits |= first_mask;
    //                     }
    //                     state = Some((chunk[last_idx as usize] - b'0') as u32);
    //                     // We have consumed all digits
    //                     break;
    //                 }
    //                 // Get the bit index of the newline
    //                 let newline_idx = newlines.trailing_zeros();
    //                 // Get the mask for all digits before the first newline
    //                 let up_to_newline = u32::MAX >> (31 - newline_idx);
    //                 // print_mask("bytes up to newline", up_to_newline);
    //                 let digits_before_newline = up_to_newline & remaining_digits;
    //                 // print_mask("digits before newline", digits_before_newline);
    //                 let (mut first_mask, last_mask) = get_first_and_last_bitmask(digits_before_newline);
    //                 if let Some(state) = state {
    //                     // We had a digit saved, that means our "first" digit isn't really the
    //                     // first.
    //                     first_mask = 0;
    //                     if last_mask == 0 {
    //                         // We didn't find a new last digit, use the one we saved
    //                         total_last += state;
    //                     }
    //                 }
    //
    //                 first_digits |= first_mask;
    //                 last_digits |= last_mask;
    //                 // The line terminates here
    //                 state = None;
    //                 // Consume all digits before newline
    //                 remaining_digits ^= digits_before_newline;
    //                 // Consume newline
    //                 newlines &= !(up_to_newline);
    //             }
    //             if newlines != 0 {
    //                 // Some newlines were not captured. Save the last state
    //                 total_last += state.take().unwrap_or(0);
    //             }
    //             let first_digits = _mm256_blendv_epi8(zero, vdigits, widen_mask(first_digits));
    //
    //             let last_digits = _mm256_blendv_epi8(zero, vdigits, widen_mask(last_digits));
    //
    //             sum_first = _mm256_add_epi8(sum_first, first_digits);
    //             sum_last = _mm256_add_epi8(sum_last, last_digits);
    //             if iter % 28 == 27 {
    //                 total_first += horizontal_sum(sum_first) as u32;
    //                 total_last += horizontal_sum(sum_last) as u32;
    //                 sum_first = zero;
    //                 sum_last = zero;
    //             }
    //         }
    //         total_first += horizontal_sum(sum_first) as u32;
    //         total_last += horizontal_sum(sum_last) as u32;
    //         let last = state.take();
    //         let rem = chunks_iter.remainder();
    //         let (first, last) = solve_iter(rem, last);
    //         total_first += first;
    //         total_last += last;
    // Process the suffix
    let (suffix_first, suffix_last, state) = solve_iter(suffix, state);
    let suffix_last = suffix_last + state.unwrap_or(0) as u32;
    first += suffix_first;
    last += suffix_last;
    first * 10 + last
    // }
}
//
// fn horizontal_sum(vector: __m256i) -> u16 {
//     unsafe {
//         let lo = _mm256_unpacklo_epi8(vector, _mm256_setzero_si256());
//         let hi = _mm256_unpackhi_epi8(vector, _mm256_setzero_si256());
//         let sum1 = _mm256_add_epi16(lo, hi);
//         let sum2 = _mm_add_epi16(
//             _mm256_extracti128_si256::<0>(sum1),
//             _mm256_extracti128_si256::<1>(sum1),
//         );
//         let sum3 = _mm_add_epi16(sum2, _mm_shuffle_epi32::<0b00001110>(sum2));
//         let sum4 = _mm_add_epi16(sum3, _mm_shuffle_epi32::<0b00000001>(sum3));
//         let sum5 = _mm_add_epi16(sum4, _mm_shufflelo_epi16::<0b00000001>(sum4));
//         _mm_extract_epi16::<0>(sum5) as u16
//     }
// }
//
// #[inline]
// fn widen_mask(mask: u32) -> __m256i {
//     unsafe {
//         let vmask1 = _mm256_set1_epi32(mask as i32);
//         let shuffle = _mm256_setr_epi64x(
//             0x0000000000000000,
//             0x0101010101010101,
//             0x0202020202020202,
//             0x0303030303030303,
//         );
//         let vmask2 = _mm256_shuffle_epi8(vmask1, shuffle);
//         let bit_mask = _mm256_set1_epi64x(0x7F_BF_DF_EF_F7_FB_FD_FE);
//         let vmask3 = _mm256_or_si256(vmask2, bit_mask);
//         _mm256_cmpeq_epi8(vmask3, _mm256_set1_epi8(-1))
//     }
// }
fn get_first_and_last_set(mask: Mask) -> Option<(u32, u32)> {
    dbg!(mask);
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
