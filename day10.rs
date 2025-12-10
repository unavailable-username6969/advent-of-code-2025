use std::fs::File;
use std::io::{BufRead, BufReader, Write};

const DEFAULT_INPUT: &str = "../input/day10.txt";
const DEFAULT_OUTPUT: &str = "output/day10.txt";
const EPS: f64 = 1e-9;

#[derive(Debug, Clone)]
struct Machine {
    buttons: Vec<Vec<usize>>,
    targets: Vec<i64>,
    light_count: usize,
}

fn parse_line(line: &str) -> Machine {
    let mut m = Machine {
        buttons: Vec::new(),
        targets: Vec::new(),
        light_count: 0,
    };

    let ls = line.find('[').unwrap();
    let le = line.find(']').unwrap();
    m.light_count = le - ls - 1;

    let mut pos = le + 1;
    let chars: Vec<char> = line.chars().collect();
    while pos < chars.len() && chars[pos] != '{' {
        if chars[pos] == '(' {
            let end = line[pos..].find(')').unwrap() + pos;
            let btn_str = &line[pos + 1..end];
            let btn: Vec<usize> = btn_str
                .split(',')
                .filter(|s| !s.is_empty())
                .map(|s| s.trim().parse().unwrap())
                .collect();
            m.buttons.push(btn);
            pos = end + 1;
        } else {
            pos += 1;
        }
    }

    let ts = line.find('{').unwrap();
    let te = line.find('}').unwrap();
    let targets_str = &line[ts + 1..te];
    m.targets = targets_str
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().parse().unwrap())
        .collect();

    m
}

fn solve_part1(line: &str, m: &Machine) -> i32 {
    let ls = line.find('[').unwrap();
    let lights: Vec<bool> = line[ls + 1..ls + 1 + m.light_count]
        .chars()
        .map(|c| c == '#')
        .collect();

    let nb = m.buttons.len();
    let mut mini = i32::MAX;

    for mask in 0..(1 << nb) {
        let presses = (mask as u32).count_ones() as i32;
        if presses >= mini {
            continue;
        }

        let mut curr = vec![false; m.light_count];
        for b in 0..nb {
            if (mask & (1 << b)) != 0 {
                for &idx in &m.buttons[b] {
                    if idx < m.light_count {
                        curr[idx] = !curr[idx];
                    }
                }
            }
        }

        let fine = curr.iter().zip(lights.iter()).all(|(&c, &l)| c == l);
        if fine {
            mini = presses;
        }
    }

    if mini == i32::MAX {
        0
    } else {
        mini
    }
}

fn solve_part2(m: &Machine) -> i64 {
    let nc = m.targets.len();
    let nb = m.buttons.len();

    if nc == 0 || nb == 0 {
        return 0;
    }

    let mut aug: Vec<Vec<f64>> = vec![vec![0.0; nb + 1]; nc];
    for (bi, btn) in m.buttons.iter().enumerate() {
        for &idx in btn {
            if idx < nc {
                aug[idx][bi] = 1.0;
            }
        }
    }
    for i in 0..nc {
        aug[i][nb] = m.targets[i] as f64;
    }

    let mut pivot_cols: Vec<i32> = vec![-1; nc];
    let mut rank = 0;

    for col in 0..nb {
        if rank >= nc {
            break;
        }

        let mut pivot: Option<usize> = None;
        for row in rank..nc {
            if aug[row][col].abs() > EPS {
                pivot = Some(row);
                break;
            }
        }

        let pivot = match pivot {
            Some(p) => p,
            None => continue,
        };

        aug.swap(rank, pivot);
        pivot_cols[rank] = col as i32;

        let piv = aug[rank][col];
        for j in 0..=nb {
            aug[rank][j] /= piv;
        }

        for row in 0..nc {
            if row != rank && aug[row][col].abs() > EPS {
                let fact = aug[row][col];
                for j in 0..=nb {
                    aug[row][j] -= fact * aug[rank][j];
                }
            }
        }
        rank += 1;
    }

    for i in rank..nc {
        if aug[i][nb].abs() > EPS {
            return 0;
        }
    }

    let mut is_pivot = vec![false; nb];
    for i in 0..rank {
        if pivot_cols[i] >= 0 {
            is_pivot[pivot_cols[i] as usize] = true;
        }
    }

    let free_cols: Vec<usize> = (0..nb).filter(|&j| !is_pivot[j]).collect();
    let free_cnt = free_cols.len();

    let mut mini: i64 = i64::MAX;
    let mut free_vals: Vec<i64> = vec![0; free_cnt];

    fn enumerate(
        idx: usize,
        free_cnt: usize,
        free_cols: &[usize],
        free_vals: &mut Vec<i64>,
        aug: &[Vec<f64>],
        pivot_cols: &[i32],
        rank: usize,
        nb: usize,
        nc: usize,
        targets: &[i64],
        buttons: &[Vec<usize>],
        mini: &mut i64,
    ) {
        if idx == free_cnt {
            let mut solution: Vec<i64> = vec![0; nb];
            for i in 0..free_cnt {
                solution[free_cols[i]] = free_vals[i];
            }

            let mut total: i64 = free_vals.iter().sum();
            if total >= *mini {
                return;
            }

            for i in 0..rank {
                let pcol = pivot_cols[i] as usize;
                let mut val = aug[i][nb];
                for j in 0..nb {
                    if j != pcol {
                        val -= aug[i][j] * solution[j] as f64;
                    }
                }
                let ival = val.round() as i64;
                if (val - ival as f64).abs() > EPS || ival < 0 {
                    return;
                }
                solution[pcol] = ival;
                total += ival;
                if total >= *mini {
                    return;
                }
            }
            *mini = total;
            return;
        }

        let mut max_val: i64 = i64::MAX;
        for &btn_idx in &buttons[free_cols[idx]] {
            if btn_idx < nc {
                max_val = max_val.min(targets[btn_idx]);
            }
        }
        if max_val == i64::MAX {
            max_val = 0;
        } else if max_val > 500 {
            max_val = 500;
        }

        for v in 0..=max_val {
            free_vals[idx] = v;
            enumerate(
                idx + 1,
                free_cnt,
                free_cols,
                free_vals,
                aug,
                pivot_cols,
                rank,
                nb,
                nc,
                targets,
                buttons,
                mini,
            );
        }
    }

    enumerate(
        0,
        free_cnt,
        &free_cols,
        &mut free_vals,
        &aug,
        &pivot_cols,
        rank,
        nb,
        nc,
        &m.targets,
        &m.buttons,
        &mut mini,
    );

    if mini == i64::MAX {
        0
    } else {
        mini
    }
}

fn part_one(filename: &str) -> i32 {
    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut ans = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        if !line.is_empty() {
            let m = parse_line(&line);
            ans += solve_part1(&line, &m);
        }
    }
    ans
}

fn part_two(filename: &str) -> i64 {
    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut ans: i64 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        if !line.is_empty() {
            let m = parse_line(&line);
            ans += solve_part2(&m);
        }
    }
    ans
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_file = if args.len() > 1 {
        &args[1]
    } else {
        DEFAULT_INPUT
    };

    println!("Reading from: {}", input_file);

    let p1 = part_one(input_file);
    let p2 = part_two(input_file);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);

    if let Ok(mut file) = File::create(DEFAULT_OUTPUT) {
        writeln!(file, "{}", p1).ok();
        writeln!(file, "{}", p2).ok();
    }
}
