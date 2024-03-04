use std::{
    fmt::Display,
    io::{stdin, Read},
    time::Duration,
};

pub fn bench<I: Clone, T, F: FnMut(I) -> T>(
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

pub fn read_input() -> String {
    let mut buf = String::with_capacity(4096);
    stdin().read_to_string(&mut buf).unwrap();
    buf
}

pub fn print_results(result: impl Display, runtime: Duration) {
    println!("{result}\n{nanos}", nanos = runtime.as_nanos())
}

pub fn hook_solution<T: Display, F: FnMut(&str) -> T>(solution: F) {
    let input = read_input();
    let (result, runtime) = bench(std::time::Duration::from_millis(128), solution, &input);
    print_results(result, runtime);
}
