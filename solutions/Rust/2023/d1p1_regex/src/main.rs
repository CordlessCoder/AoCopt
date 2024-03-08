use std::sync::OnceLock;

static REGEX: OnceLock<[regex::bytes::Regex; 2]> = OnceLock::new();

fn solve(input: &str) -> u32 {
    let [first, last] = REGEX.get().unwrap();
    let first = first
        .captures_iter(input.as_bytes())
        .map(|cap| cap.get(1).unwrap().as_bytes()[0])
        .map(|b| b.wrapping_sub(b'0'));
    let last = last
        .captures_iter(input.as_bytes())
        .map(|cap| cap.get(1).unwrap().as_bytes()[0])
        .map(|b| b.wrapping_sub(b'0'));
    let lines = first.zip(last);

    let (sum_first, sum_last) = lines.fold((0, 0), |(sum_first, sum_last), (first, last)| {
        (sum_first + first as u32, sum_last + last as u32)
    });

    sum_first * 10 + sum_last
}

fn main() {
    REGEX.get_or_init(|| {
        let first = regex::bytes::RegexBuilder::new(r#"^\D*(\d)"#)
            .unicode(false)
            .multi_line(true)
            .build()
            .unwrap();
        let last = regex::bytes::RegexBuilder::new(r#"(\d)\D*$"#)
            .unicode(false)
            .multi_line(true)
            .build()
            .unwrap();
        [first, last]
    });
    aoc_util::hook_solution(solve)
}
