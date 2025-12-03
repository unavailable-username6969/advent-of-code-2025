//! https://adventofcode.com/2025/day/2
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;

const DEFAULT_INPUT: &str = "../input/day02.txt";
const DEFAULT_OUTPUT: &str = "output/day02.txt";

const MAX_LIMIT: i64 = 100_000_000_000;

fn parse_ranges(content: &str) -> Vec<(i64, i64)> {
    let clean_content: String = content.chars().filter(|c| !c.is_whitespace()).collect();
    let mut ranges = Vec::new();

    for segment in clean_content.split(',') {
        if segment.is_empty() {
            continue;
        }
        if let Some((s_str, e_str)) = segment.split_once('-') {
            if let (Ok(s), Ok(e)) = (s_str.parse::<i64>(), e_str.parse::<i64>()) {
                ranges.push((s, e));
            }
        }
    }
    ranges
}

fn generate_part1_candidates() -> Vec<i64> {
    let mut candidates = Vec::new();
    for len in 1..=5 {
        let start = 10_i64.pow(len - 1);
        let end = 10_i64.pow(len) - 1;
        let multiplier = 10_i64.pow(len) + 1;

        for seed in start..=end {
            candidates.push(seed * multiplier);
        }
    }
    candidates.sort_unstable();
    candidates
}

fn generate_part2_candidates() -> Vec<i64> {
    let mut unique_nums = HashSet::new();

    for len in 1..=6 {
        let start = 10_i64.pow(len - 1);
        let end = 10_i64.pow(len) - 1;

        for seed in start..=end {
            let s_str = seed.to_string();
            let mut current_s = s_str.clone();

            current_s.push_str(&s_str);

            loop {
                if let Ok(val) = current_s.parse::<i64>() {
                    if val > MAX_LIMIT {
                        break;
                    }
                    unique_nums.insert(val);
                } else {
                    break;
                }

                current_s.push_str(&s_str);
                if current_s.len() > 12 {
                    break;
                }
            }
        }
    }

    let mut candidates: Vec<i64> = unique_nums.into_iter().collect();
    candidates.sort_unstable();
    candidates
}

fn solve(ranges: &[(i64, i64)], candidates: &[i64]) -> i64 {
    let mut total_sum = 0;

    for &(r_start, r_end) in ranges {
        let start_idx = candidates.partition_point(|&x| x < r_start);

        let end_idx = candidates.partition_point(|&x| x <= r_end);

        if start_idx < candidates.len() {
            total_sum += candidates[start_idx..end_idx].iter().sum::<i64>();
        }
    }
    total_sum
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = args.get(1).map(|s| s.as_str()).unwrap_or(DEFAULT_INPUT);
    println!("Reading from: {}", input_path);

    let content = match fs::read_to_string(input_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let ranges = parse_ranges(&content);

    let p1_candidates = generate_part1_candidates();
    let p2_candidates = generate_part2_candidates();

    let p1 = solve(&ranges, &p1_candidates);
    let p2 = solve(&ranges, &p2_candidates);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);

    if let Some(parent) = Path::new(DEFAULT_OUTPUT).parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::write(DEFAULT_OUTPUT, format!("{}\n{}\n", p1, p2));
}
