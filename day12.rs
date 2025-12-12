use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

const DEFAULT_INPUT: &str = "../input/day12.txt";
const DEFAULT_OUTPUT: &str = "output/day12.txt";

fn part_one(filename: &str) -> io::Result<i32> {
    let path = Path::new(filename);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut shape_sizes: Vec<usize> = Vec::new();
    let mut current_shape_id: Option<usize> = None;
    let mut ans = 0;

    for line_result in reader.lines() {
        let line = line_result?;
        let trimmed = line.trim();

        if trimmed.is_empty() {
            continue;
        }

        if let Some(colon_idx) = trimmed.find(':') {
            let prefix = &trimmed[..colon_idx];
            let suffix = &trimmed[colon_idx + 1..];

            if prefix.contains('x') {
                let dims: Vec<&str> = prefix.split('x').collect();
                if dims.len() == 2 {
                    let w: usize = dims[0].parse().unwrap_or(0);
                    let h: usize = dims[1].parse().unwrap_or(0);
                    let region_area = w * h;

                    let counts: Vec<usize> = suffix
                        .split_whitespace()
                        .map(|s| s.parse().unwrap_or(0))
                        .collect();

                    let mut presents_area = 0;
                    for (idx, &count) in counts.iter().enumerate() {
                        if idx < shape_sizes.len() {
                            presents_area += count * shape_sizes[idx];
                        }
                    }

                    if presents_area <= region_area {
                        ans += 1;
                    }
                }
            } else if let Ok(id) = prefix.parse::<usize>() {
                current_shape_id = Some(id);
                if shape_sizes.len() <= id {
                    shape_sizes.resize(id + 1, 0);
                }
            }
        } else if let Some(id) = current_shape_id {
            let block_count = trimmed.chars().filter(|&c| c == '#').count();
            if id < shape_sizes.len() {
                shape_sizes[id] += block_count;
            }
        }
    }

    Ok(ans)
}

fn part_two(_filename: &str) -> io::Result<i32> {
    Ok(0)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let input_file = if args.len() > 1 {
        &args[1]
    } else {
        DEFAULT_INPUT
    };

    println!("Reading from: {}", input_file);

    let p1 = part_one(input_file).unwrap_or(-1);
    let p2 = part_two(input_file).unwrap_or(-1);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);

    let mut output = File::create(DEFAULT_OUTPUT)?;
    writeln!(output, "{}", p1)?;
    writeln!(output, "{}", p2)?;

    Ok(())
}
