use std::{
    hint::black_box,
    io::{stdin, Read},
};

fn bench<I: Clone, T, F: FnMut(I) -> T>(
    runtime: std::time::Duration,
    mut function: F,
    input: I,
) -> (T, std::time::Duration) {
    use std::time::*;
    const MIN_SAMPLES: u32 = 100;
    let start = Instant::now();
    let mut res = function(input.clone());
    let oneshot = start.elapsed();
    let runs = runtime.as_secs_f64() / oneshot.as_secs_f64();
    let per_sample = runs as u32 / MIN_SAMPLES;
    let per_sample = per_sample.min(32).max(2);
    macro_rules! sample {
        () => {
            for _ in 0..per_sample {
                res = function(std::hint::black_box(input.clone()));
            }
        };
    }
    let start = Instant::now();
    sample!();
    let sample_time = start.elapsed();
    let samples = runtime.as_secs_f64() / sample_time.as_secs_f64();
    let samples = samples.max(1.0) as u32;
    let mut took = Duration::ZERO;
    for _ in 0..samples {
        let start = Instant::now();
        sample!();
        let sample_time = start.elapsed();
        took += sample_time;
    }
    (res, took / samples / per_sample)
}

fn solution(input: &str) -> u32 {
    let (first, last) = input
        .as_bytes()
        .split(|&b| b == b'\n')
        .map(|line| {
            let mut iter = line.iter().copied();
            let first = iter.find(|&b| b <= b'9').unwrap_or(0);
            let last = iter.rfind(|&b| b <= b'9').unwrap_or(first);
            ((first & 0xf) as u32, (last & 0xf) as u32)
        })
        .fold((0, 0), |(first, last), (f, l)| (first + f, last + l));
    first * 10 + last
}

fn main() {
    let mut buf = String::with_capacity(4096);
    stdin().read_to_string(&mut buf).unwrap();
    _ = solution(black_box(&buf));
    let (res, took) = bench(std::time::Duration::from_millis(128), solution, &buf);
    println!("{res}\n{nanos}", nanos = took.as_nanos())
}
