use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = fs::File::open(filename)?;
    let buf = io::BufReader::new(file);
    buf.lines().collect()
}

fn solve() {
    let args: Vec<String> = env::args().collect();
    let default_input = "../input/day07.txt";
    let input_path = if args.len() > 1 {
        &args[1]
    } else {
        default_input
    };

    println!("Reading from: {}", input_path);

    let lines = match read_lines(input_path) {
        Ok(l) => l,
        Err(_) => {
            eprintln!("Error reading file: {}", input_path);
            return;
        }
    };

    let grid: Vec<Vec<char>> = lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect();

    if grid.is_empty() {
        println!("Grid is empty");
        return;
    }

    let height = grid.len();
    let width = grid[0].len();
    let mut start_pos: Option<(usize, usize)> = None;

    for r in 0..height {
        for c in 0..width {
            if grid[r][c] == 'S' {
                start_pos = Some((r, c));
                break;
            }
        }
        if start_pos.is_some() {
            break;
        }
    }

    let (start_r, start_c) = match start_pos {
        Some(pos) => pos,
        None => {
            eprintln!("Could not find start point 'S'");
            return;
        }
    };

    let mut current_beams: Vec<usize> = vec![start_c];
    let mut total_splits = 0;

    for r in start_r..height {
        if current_beams.is_empty() {
            break;
        }

        let mut next_beams: Vec<usize> = Vec::new();

        for &c in &current_beams {
            if c >= width {
                continue;
            }

            let cell = grid[r][c];

            if cell == '^' {
                total_splits += 1;
                if c > 0 {
                    next_beams.push(c - 1);
                }
                if c + 1 < width {
                    next_beams.push(c + 1);
                }
            } else {
                next_beams.push(c);
            }
        }

        next_beams.sort_unstable();
        next_beams.dedup();
        current_beams = next_beams;
    }

    println!("Part 1: {}", total_splits);

    let mut counts: Vec<u64> = vec![0; width];
    counts[start_c] = 1;

    for r in start_r..height {
        let mut next_counts: Vec<u64> = vec![0; width];

        for c in 0..width {
            if counts[c] == 0 {
                continue;
            }

            let cell = grid[r][c];
            let count = counts[c];

            if cell == '^' {
                if c > 0 {
                    next_counts[c - 1] += count;
                }
                if c + 1 < width {
                    next_counts[c + 1] += count;
                }
            } else {
                next_counts[c] += count;
            }
        }

        counts = next_counts;
    }

    let total_timelines: u64 = counts.iter().sum();
    println!("Part 2: {}", total_timelines);

    let output_path = "output/day07.txt";
    if let Some(parent) = Path::new(output_path).parent() {
        let _ = fs::create_dir_all(parent);
    }
    let output_content = format!("{}\n{}\n", total_splits, total_timelines);
    if let Err(_) = fs::write(output_path, output_content) {}
}

fn main() {
    solve();
}
