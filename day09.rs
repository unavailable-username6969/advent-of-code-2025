use std::cmp::{max, min};
use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

fn is_between(v: i64, a: i64, b: i64) -> bool {
    v > min(a, b) && v < max(a, b)
}

fn intervals_overlap(a: i64, b: i64, c: i64, d: i64) -> bool {
    max(min(a, b), min(c, d)) < min(max(a, b), max(c, d))
}

fn on_segment(px: f64, py: f64, a: Point, b: Point) -> bool {
    let ax = a.x as f64;
    let ay = a.y as f64;
    let bx = b.x as f64;
    let by = b.y as f64;

    if a.x == b.x {
        (px - ax).abs() < 1e-9 && py >= ay.min(by) && py <= ay.max(by)
    } else {
        (py - ay).abs() < 1e-9 && px >= ax.min(bx) && px <= ax.max(bx)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let default_input = "../input/day09.txt";
    let input_file = if args.len() > 1 {
        &args[1]
    } else {
        default_input
    };

    println!("Reading from: {}", input_file);

    let content = match fs::read_to_string(input_file) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Error: Could not open file {}", input_file);
            return;
        }
    };

    let points: Vec<Point> = content
        .lines()
        .filter_map(|line| {
            if line.trim().is_empty() {
                return None;
            }
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() >= 2 {
                let x = parts[0].trim().parse::<i64>().ok()?;
                let y = parts[1].trim().parse::<i64>().ok()?;
                Some(Point { x, y })
            } else {
                None
            }
        })
        .collect();

    let n = points.len();
    let mut p1: i64 = 0;
    let mut p2: i64 = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            let p_i = points[i];
            let p_j = points[j];

            let width = (p_i.x - p_j.x).abs() + 1;
            let height = (p_i.y - p_j.y).abs() + 1;
            let area = width * height;

            if area > p1 {
                p1 = area;
            }

            if area <= p2 {
                continue;
            }

            let min_x = min(p_i.x, p_j.x);
            let max_x = max(p_i.x, p_j.x);
            let min_y = min(p_i.y, p_j.y);
            let max_y = max(p_i.y, p_j.y);

            let mut cuts_through = false;
            for k in 0..n {
                let a = points[k];
                let b = points[(k + 1) % n];

                if a.x == b.x {
                    if is_between(a.x, min_x, max_x) && intervals_overlap(a.y, b.y, min_y, max_y) {
                        cuts_through = true;
                    }
                } else if is_between(a.y, min_y, max_y) && intervals_overlap(a.x, b.x, min_x, max_x)
                {
                    cuts_through = true;
                }

                if cuts_through {
                    break;
                }
            }

            if cuts_through {
                continue;
            }

            let cx = (p_i.x + p_j.x) as f64 / 2.0;
            let cy = (p_i.y + p_j.y) as f64 / 2.0;

            let mut on_boundary = false;
            let mut intersections = 0;

            for k in 0..n {
                let a = points[k];
                let b = points[(k + 1) % n];

                if on_segment(cx, cy, a, b) {
                    on_boundary = true;
                    break;
                }

                let ay = a.y as f64;
                let by = b.y as f64;
                let ax = a.x as f64;
                let bx = b.x as f64;

                if (ay > cy) != (by > cy) {
                    let edge_x = (bx - ax) * (cy - ay) / (by - ay) + ax;
                    if cx < edge_x {
                        intersections += 1;
                    }
                }
            }

            if on_boundary || (intersections % 2 == 1) {
                p2 = area;
            }
        }
    }

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);

    let output_path = Path::new("output/day09.txt");
    if let Some(parent) = output_path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let output_content = format!("{}\n{}\n", p1, p2);
    if let Err(e) = fs::write(output_path, output_content) {
        eprintln!("Error writing output: {}", e);
    }
}
