//! https://adventofcode.com/2025/day/5
use std::cmp;
use std::env;
use std::fs;
use std::path::Path;
use std::str::FromStr;

const DEFAULT_INPUT: &str = "../input/day05.txt";
const DEFAULT_OUTPUT: &str = "output/day05.txt";

#[derive(Debug, Clone, Copy)]
struct Range {
    start: i64,
    end: i64,
}

#[derive(Debug)]
enum InputEntry {
    Range(Range),
    Id(i64),
}

impl FromStr for InputEntry {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.is_empty() {
            return Err("Empty line".to_string());
        }

        if let Some(pos) = s.find('-') {
            let start = s[..pos].parse::<i64>().map_err(|_| "Invalid start")?;
            let end = s[pos + 1..].parse::<i64>().map_err(|_| "Invalid end")?;
            Ok(InputEntry::Range(Range { start, end }))
        } else {
            let id = s.parse::<i64>().map_err(|_| "Invalid ID")?;
            Ok(InputEntry::Id(id))
        }
    }
}

fn part_one(entries: &[InputEntry]) -> i64 {
    let mut ranges = Vec::new();
    let mut ids = Vec::new();

    for entry in entries {
        match entry {
            InputEntry::Range(r) => ranges.push(r),
            InputEntry::Id(id) => ids.push(*id),
        }
    }

    let mut fresh_count = 0;
    for id in ids {
        for r in &ranges {
            if id >= r.start && id <= r.end {
                fresh_count += 1;
                break;
            }
        }
    }
    fresh_count
}

fn part_two(entries: &[InputEntry]) -> i64 {
    let mut ranges: Vec<Range> = entries
        .iter()
        .filter_map(|e| match e {
            InputEntry::Range(r) => Some(*r),
            _ => None,
        })
        .collect();

    if ranges.is_empty() {
        return 0;
    }

    ranges.sort_by_key(|r| r.start);

    let mut merged = Vec::new();
    merged.push(ranges[0]);

    for r in ranges.iter().skip(1) {
        let last_idx = merged.len() - 1;
        let last = &mut merged[last_idx];

        if r.start <= last.end {
            last.end = cmp::max(last.end, r.end);
        } else {
            merged.push(*r);
        }
    }
  
    let mut total_fresh: i64 = 0;
    for r in merged {
        total_fresh += r.end - r.start + 1;
    }

    total_fresh
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = args.get(1).map(|s| s.as_str()).unwrap_or(DEFAULT_INPUT);
    println!("Reading from: {}", input_path);

    let content = fs::read_to_string(input_path).expect("Could not read input file");
    let entries: Vec<InputEntry> = content.lines().filter_map(|l| l.parse().ok()).collect();

    let p1 = part_one(&entries);
    let p2 = part_two(&entries);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);

    if let Some(parent) = Path::new(DEFAULT_OUTPUT).parent() {
        let _ = fs::create_dir_all(parent);
    }
    fs::write(DEFAULT_OUTPUT, format!("{}\n{}\n", p1, p2)).expect("Could not write output file");

    println!("Results saved to {}", DEFAULT_OUTPUT);
}
