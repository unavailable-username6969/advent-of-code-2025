use std::fs::{self, File};
use std::io::{self, Write};

const DEFAULT_INPUT: &str = "../input/day08.txt";
const DEFAULT_OUTPUT: &str = "output/day08.txt";

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug, Clone)]
struct Edge {
    u: usize,
    v: usize,
    dist_sq: i64,
}

struct DSU {
    parent: Vec<usize>,
    size: Vec<usize>,
    num_components: usize,
}

impl DSU {
    fn new(n: usize) -> Self {
        DSU {
            parent: (0..n).collect(),
            size: vec![1; n],
            num_components: n,
        }
    }

    fn find(&mut self, i: usize) -> usize {
        let mut root = i;
        while root != self.parent[root] {
            root = self.parent[root];
        }

        let mut curr = i;
        while curr != root {
            let next = self.parent[curr];
            self.parent[curr] = root;
            curr = next;
        }
        root
    }

    fn union(&mut self, i: usize, j: usize) -> bool {
        let root_i = self.find(i);
        let root_j = self.find(j);

        if root_i != root_j {
            if self.size[root_i] < self.size[root_j] {
                self.parent[root_i] = root_j;
                self.size[root_j] += self.size[root_i];
            } else {
                self.parent[root_j] = root_i;
                self.size[root_i] += self.size[root_j];
            }
            self.num_components -= 1;
            true
        } else {
            false
        }
    }
}

fn parse_input(filename: &str) -> Vec<Point> {
    let content = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Error: Could not open file {}", filename);
            return Vec::new();
        }
    };

    content
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
            if parts.len() == 3 {
                let x = parts[0].parse::<i64>().ok()?;
                let y = parts[1].parse::<i64>().ok()?;
                let z = parts[2].parse::<i64>().ok()?;
                Some(Point { x, y, z })
            } else {
                None
            }
        })
        .collect()
}

fn generate_edges(points: &[Point]) -> Vec<Edge> {
    let n = points.len();
    if n == 0 {
        return Vec::new();
    }

    let mut edges = Vec::with_capacity(n * (n - 1) / 2);

    for i in 0..n {
        for j in (i + 1)..n {
            let dx = points[i].x - points[j].x;
            let dy = points[i].y - points[j].y;
            let dz = points[i].z - points[j].z;
            let dist_sq = dx * dx + dy * dy + dz * dz;
            edges.push(Edge {
                u: i,
                v: j,
                dist_sq,
            });
        }
    }
    edges
}

fn part_one(filename: &str) -> i64 {
    let points = parse_input(filename);
    if points.is_empty() {
        return 0;
    }

    let mut edges = generate_edges(&points);

    edges.sort_by_key(|e| e.dist_sq);

    let k = 1000.min(edges.len());

    let mut dsu = DSU::new(points.len());
    for edge in edges.iter().take(k) {
        dsu.union(edge.u, edge.v);
    }

    let mut sizes: Vec<i64> = (0..points.len())
        .filter(|&i| dsu.parent[i] == i)
        .map(|i| dsu.size[i] as i64)
        .collect();

    sizes.sort_by(|a, b| b.cmp(a));

    sizes.iter().take(3).product()
}

fn part_two(filename: &str) -> i64 {
    let points = parse_input(filename);
    if points.is_empty() {
        return 0;
    }

    let mut edges = generate_edges(&points);

    edges.sort_by_key(|e| e.dist_sq);

    let mut dsu = DSU::new(points.len());

    for edge in edges {
        if dsu.union(edge.u, edge.v) && dsu.num_components == 1 {
            return points[edge.u].x * points[edge.v].x;
        }
    }
    0
}

fn main() -> io::Result<()> {
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

    let mut file = File::create(DEFAULT_OUTPUT)?;
    writeln!(file, "{}\n{}", p1, p2)?;

    Ok(())
}
