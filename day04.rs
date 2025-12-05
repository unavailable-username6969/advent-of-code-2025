use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

const DEFAULT_INPUT: &str = "../input/day04.txt";
const DEFAULT_OUTPUT: &str = "output/day04.txt";

fn read_grid(filename: &str) -> Result<Vec<Vec<char>>, std::io::Error> {
    let content = fs::read_to_string(filename)?;
    let grid: Vec<Vec<char>> = content
        .lines()
        .map(|line| line.trim_end().chars().collect())
        .filter(|row: &Vec<char>| !row.is_empty())
        .collect();
    Ok(grid)
}

fn count_neighbors(grid: &[Vec<char>], r: usize, c: usize) -> usize {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    let mut count = 0;

    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }

            let nr = r as isize + dr;
            let nc = c as isize + dc;

            if nr >= 0 && nr < rows && nc >= 0 && nc < cols {
                if grid[nr as usize][nc as usize] == '@' {
                    count += 1;
                }
            }
        }
    }
    count
}

fn part_one(filename: &str) -> io::Result<usize> {
    let grid = read_grid(filename)?;
    if grid.is_empty() {
        return Ok(0);
    }

    let rows = grid.len();
    let cols = grid[0].len();
    let mut accessible_rolls = 0;

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == '@' {
                if count_neighbors(&grid, r, c) < 4 {
                    accessible_rolls += 1;
                }
            }
        }
    }

    Ok(accessible_rolls)
}

fn part_two(filename: &str) -> io::Result<usize> {
    let mut grid = read_grid(filename)?;
    if grid.is_empty() {
        return Ok(0);
    }

    let rows = grid.len();
    let cols = grid[0].len();
    let mut total_removed = 0;

    loop {
        let mut to_remove = Vec::new();

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == '@' {
                    if count_neighbors(&grid, r, c) < 4 {
                        to_remove.push((r, c));
                    }
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        total_removed += to_remove.len();
        for (r, c) in to_remove {
            grid[r][c] = '.';
        }
    }

    Ok(total_removed)
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
