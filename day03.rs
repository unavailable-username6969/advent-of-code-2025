use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

const DEFAULT_INPUT: &str = "../input/day03.txt";
const DEFAULT_OUTPUT: &str = "output/day03.txt";

fn read_grid(filename: &str) -> Result<Vec<Vec<char>>, std::io::Error> {
    let content = fs::read_to_string(filename)?;
    let grid: Vec<Vec<char>> = content
        .lines()
        .map(|line| line.trim_end().chars().collect())
        .filter(|row: &Vec<char>| !row.is_empty())
        .collect();
    Ok(grid)
}

fn solve_bank(bank: &[char], count: usize) -> u64 {
    let n = bank.len();
    if n < count {
        return 0;
    }

    let mut current_idx = 0;
    let mut result = String::with_capacity(count);

    for needed in (1..=count).rev() {
        let end_search = n - needed;

        let mut max_digit = '\0';
        let mut max_idx = current_idx;

        for i in current_idx..=end_search {
            if bank[i] > max_digit {
                max_digit = bank[i];
                max_idx = i;
            }
        }

        result.push(max_digit);

        current_idx = max_idx + 1;
    }

    result.parse::<u64>().unwrap_or(0)
}

fn part_one(filename: &str) -> io::Result<u64> {
    let grid = read_grid(filename)?;
    if grid.is_empty() {
        return Ok(0);
    }

    let mut total_joltage = 0;
    for row in grid {
        total_joltage += solve_bank(&row, 2);
    }

    Ok(total_joltage)
}

fn part_two(filename: &str) -> io::Result<u64> {
    let grid = read_grid(filename)?;
    if grid.is_empty() {
        return Ok(0);
    }

    let mut total_joltage = 0;
    for row in grid {
        total_joltage += solve_bank(&row, 12);
    }

    Ok(total_joltage)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input_file = if args.len() > 1 {
        &args[1]
    } else {
        DEFAULT_INPUT
    };

    println!("Reading from: {}", input_file);

    // Part 1
    match part_one(input_file) {
        Ok(p1) => {
            println!("Part 1: {}", p1);

            // Part 2
            match part_two(input_file) {
                Ok(p2) => {
                    println!("Part 2: {}", p2);

                    if let Some(parent) = Path::new(DEFAULT_OUTPUT).parent() {
                        fs::create_dir_all(parent)?;
                    }
                    let mut file = fs::File::create(DEFAULT_OUTPUT)?;
                    writeln!(file, "{}", p1)?;
                    writeln!(file, "{}", p2)?;
                }
                Err(e) => eprintln!("Error running Part 2: {}", e),
            }
        }
        Err(e) => eprintln!("Error running Part 1: {}", e),
    }

    Ok(())
}
