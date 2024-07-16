use aoc_util::*;
fn main() {
    hook_solution(solution);
}

fn solution(input: &str) -> u64 {
    let input: Vec<String> = input.lines().map(String::from).collect();
    let mut sum = 0;
    for line in input {
        sum += parse_line(&line);
    }
    sum
}

fn parse_line(line: &String) -> u64{
    let mut values: Vec<u64> = Vec::new();  

    for c in line.as_bytes().iter() {
        if c.is_ascii_digit() {  
            values.push((c - b'0') as u64);  
        }
    }

    (values[0] * 10) + values[values.len() - 1]

}