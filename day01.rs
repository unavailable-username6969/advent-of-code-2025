//! https://adventofcode.com/2025/day/1
use std::env;
use std::fs;
use std::path::Path;
use std::str::FromStr;

const DEFAULT_INPUT: &str = "../day01.txt";
const DEFAULT_OUTPUT: &str = "../output/day01.txt";
const START_POS: i32 = 50;
const DIAL_SIZE: i32 = 100;

#[derive(Debug)]
enum Dir {
    Left,
    Right,
}

struct Instruction {
    dir: Dir,
    val: i32,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.len() < 2 {
            return Err("Line too short".to_string());
        }
        let (d_str, v_str) = s.split_at(1);
        let dir = match d_str {
            "R" => Dir::Right,
            "L" => Dir::Left,
            _ => return Err(format!("Unknown direction: {}", d_str)),
        };
        let val = v_str.parse::<i32>().map_err(|_| "Invalid number")?;
        Ok(Instruction { dir, val })
    }
}

fn part_one(instructions: &[Instruction]) -> i32 {
    let (_, ans) = instructions
        .iter()
        .fold((START_POS, 0), |(pos, count), inst| {
            let next_pos = match inst.dir {
                Dir::Right => (pos + inst.val).rem_euclid(DIAL_SIZE),
                Dir::Left => (pos - inst.val).rem_euclid(DIAL_SIZE),
            };

            let new_count = if next_pos == 0 { count + 1 } else { count };

            (next_pos, new_count)
        });
    ans
}

fn part_two(instructions: &[Instruction]) -> i32 {
    // Fold state: (current_position, pass_count)
    let (_, ans) = instructions
        .iter()
        .fold((START_POS, 0), |(pos, count), inst| {
            let (dist_to_zero, next_pos);

            match inst.dir {
                Dir::Right => {
                    let gap = (DIAL_SIZE - pos).rem_euclid(DIAL_SIZE);
                    dist_to_zero = if gap == 0 { DIAL_SIZE } else { gap };
                    next_pos = (pos + inst.val).rem_euclid(DIAL_SIZE);
                }
                Dir::Left => {
                    dist_to_zero = if pos == 0 { DIAL_SIZE } else { pos };
                    next_pos = (pos - inst.val).rem_euclid(DIAL_SIZE);
                }
            };

            let mut hits = 0;
            if inst.val >= dist_to_zero {
                hits = 1 + (inst.val - dist_to_zero) / DIAL_SIZE;
            }

            (next_pos, count + hits)
        });
    ans
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = args.get(1).map(|s| s.as_str()).unwrap_or(DEFAULT_INPUT);
    println!("Reading from: {}", input_path);

    let content = fs::read_to_string(input_path).expect("Could not read input file");
    let instructions: Vec<Instruction> = content.lines().filter_map(|l| l.parse().ok()).collect();

    let p1 = part_one(&instructions);
    let p2 = part_two(&instructions);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);

    if let Some(parent) = Path::new(DEFAULT_OUTPUT).parent() {
        let _ = fs::create_dir_all(parent);
    }
    fs::write(DEFAULT_OUTPUT, format!("{}\n{}\n", p1, p2)).expect("Could not write output file");

    println!("Results saved to {}", DEFAULT_OUTPUT);
}
