use winnow::{
    ascii::{dec_uint, space0},
    combinator,
    error::InputError,
};

fn is_safe(mut levels: impl Iterator<Item = u32>, mut pred: impl FnMut(u32, u32) -> bool) -> bool {
    let mut grandprev = None;
    let Some(mut prev) = levels.next() else {
        return true;
    };
    while let Some(n) = levels.next() {
        if pred(prev, n) {
            grandprev = Some(prev);
            prev = n;
            continue;
        }
        let Some(next) = levels.next() else {
            return true;
        };
        // remove prev
        if grandprev.is_none_or(|gp| pred(gp, n)) && pred(n, next) {
            prev = next;
            break;
        }
        // remove n
        if pred(prev, next) {
            prev = next;
            break;
        }
        // can't remove either
        return false;
    }
    levels.all(|n| {
        let fine = pred(prev, n);
        prev = n;
        fine
    })
}
fn solution(mut input: &str) -> u32 {
    let mut count = 0;
    let mut buf = Vec::new();
    while !input.is_empty() {
        buf.clear();
        let num = combinator::preceded(space0, dec_uint::<_, u32, _>);
        let mut report = combinator::iterator::<_, _, InputError<_>, _>(input, num);
        buf.extend(&mut report);
        input = report.finish().unwrap().0;
        input = input.trim_start();
        let safe = is_safe(buf.iter().copied(), |a, b| {
            a < b && (1..=3).contains(&(b - a))
        }) || is_safe(buf.iter().copied(), |a, b| {
            a > b && (1..=3).contains(&(a - b))
        });
        count += safe as u32;
    }
    count
}

fn main() {
    aoc_util::hook_solution(solution);
}
