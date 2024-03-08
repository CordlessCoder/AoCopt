use std::sync::OnceLock;

static REGEX: OnceLock<regex::bytes::Regex> = OnceLock::new();

fn solve(input: &str) -> u32 {
    let regex = REGEX.get().expect("Regex was not compiled");
    let lines = regex
        .captures_iter(input.as_bytes())
        .map(|cap| {
            let first = cap.get(1).unwrap();
            let last = cap.get(2).unwrap_or(first);
            (first.as_bytes(), last.as_bytes())
        })
        .map(|(first, last)| (first[0], last[0]))
        .map(|(first, last)| (first.wrapping_sub(b'0'), last.wrapping_sub(b'0')));

    let (sum_first, sum_last) = lines.fold((0, 0), |(sum_first, sum_last), (first, last)| {
        (sum_first + first as u32, sum_last + last as u32)
    });
    sum_first * 10 + sum_last
}

fn main() {
    REGEX.get_or_init(|| {
        regex::bytes::RegexBuilder::new(r#"^\D*(\d)(?:.*(\d))?.*$"#)
            .unicode(false)
            .multi_line(true)
            .build()
            .unwrap()
    });
    aoc_util::hook_solution(solve)
}
