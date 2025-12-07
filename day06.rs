use std::env;
use std::fs;
use std::path::Path;

const DEFAULT_INPUT: &str = "../input/day06.txt";
const DEFAULT_OUTPUT: &str = "output/day06.txt";

fn solve_part1(grid: &Vec<Vec<char>>, start_col: usize, end_col: usize) -> i64 {
    let mut s = String::new();
    for row in grid {
        for col in start_col..end_col {
            s.push(row[col]);
        }
        s.push(' ');
    }

    let tokens: Vec<&str> = s.split_whitespace().collect();
    if tokens.len() < 2 {
        return 0;
    }

    let op_str = tokens.last().unwrap();
    let op_char = op_str.chars().next().unwrap_or(' ');
    if op_char != '+' && op_char != '*' {
        return 0;
    }

    let mut nums = Vec::new();
    for i in 0..tokens.len() - 1 {
        if let Ok(n) = tokens[i].parse::<i64>() {
            nums.push(n);
        }
    }

    if nums.is_empty() {
        return 0;
    }

    let mut result = nums[0];
    for &n in &nums[1..] {
        match op_char {
            '+' => result += n,
            '*' => result *= n,
            _ => {}
        }
    }
    result
}

fn solve_part2(grid: &Vec<Vec<char>>, start_col: usize, end_col: usize) -> i64 {
    let mut nums = Vec::new();
    let mut op = None;

    for x in (start_col..end_col).rev() {
        let mut num_str = String::new();
        for row in grid {
            let c = row[x];
            if c.is_ascii_digit() {
                num_str.push(c);
            } else if c == '+' || c == '*' {
                op = Some(c);
            }
        }

        if !num_str.is_empty() {
            if let Ok(n) = num_str.parse::<i64>() {
                nums.push(n);
            }
        }
    }

    if nums.is_empty() || op.is_none() {
        return 0;
    }
    let op_char = op.unwrap();

    let mut result = nums[0];
    for &n in &nums[1..] {
        match op_char {
            '+' => result += n,
            '*' => result *= n,
            _ => {}
        }
    }
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = if args.len() > 1 {
        &args[1]
    } else {
        DEFAULT_INPUT
    };
    println!("Reading from: {}", input_path);

    let content = match fs::read_to_string(input_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let lines: Vec<&str> = content.lines().collect();
    if lines.is_empty() {
        return;
    }

    let max_width = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in lines {
        let mut row: Vec<char> = line.chars().collect();
        while row.len() < max_width {
            row.push(' ');
        }
        grid.push(row);
    }

    let mut total_p1: i64 = 0;
    let mut total_p2: i64 = 0;
    let mut start_col: Option<usize> = None;

    let is_empty = |col: usize, g: &Vec<Vec<char>>| -> bool { g.iter().all(|row| row[col] == ' ') };

    for x in 0..=max_width {
        let empty = if x == max_width {
            true
        } else {
            is_empty(x, &grid)
        };

        match (start_col, empty) {
            (None, false) => {
                start_col = Some(x);
            }
            (Some(start), true) => {
                total_p1 += solve_part1(&grid, start, x);
                total_p2 += solve_part2(&grid, start, x);
                start_col = None;
            }
            _ => {}
        }
    }

    println!("Part 1: {}", total_p1);
    println!("Part 2: {}", total_p2);

    if let Some(parent) = Path::new(DEFAULT_OUTPUT).parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::write(DEFAULT_OUTPUT, format!("{}\n{}\n", total_p1, total_p2));
}
