use std::collections::HashMap;
use std::env;
use std::fs;

type Graph = HashMap<String, Vec<String>>;
type Memo = HashMap<String, u64>;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 {
        &args[1]
    } else {
        "../input/day11.txt"
    };

    let content = fs::read_to_string(filename).expect("Could not read input file");

    let graph = parse_input(&content);

    let part1 = count_paths_memoized("you", "out", &graph);
    println!("Part 1: {}", part1);

    let path_a = count_paths_memoized("svr", "dac", &graph)
        * count_paths_memoized("dac", "fft", &graph)
        * count_paths_memoized("fft", "out", &graph);

    let path_b = count_paths_memoized("svr", "fft", &graph)
        * count_paths_memoized("fft", "dac", &graph)
        * count_paths_memoized("dac", "out", &graph);

    println!("Part 2: {}", path_a + path_b);
}

fn parse_input(input: &str) -> Graph {
    let mut graph = HashMap::new();

    for line in input.lines() {
        if let Some((src, dests)) = line.split_once(':') {
            let src = src.trim().to_string();
            let dests: Vec<String> = dests.split_whitespace().map(|s| s.to_string()).collect();
            graph.insert(src, dests);
        }
    }

    graph
}

fn count_paths_memoized(start: &str, end: &str, graph: &Graph) -> u64 {
    let mut memo = HashMap::new();
    dfs(start, end, graph, &mut memo)
}

fn dfs(current: &str, target: &str, graph: &Graph, memo: &mut Memo) -> u64 {
    if current == target {
        return 1;
    }

    if let Some(&count) = memo.get(current) {
        return count;
    }

    let mut count = 0;
    if let Some(neighbors) = graph.get(current) {
        for next in neighbors {
            count += dfs(next, target, graph, memo);
        }
    }

    memo.insert(current.to_string(), count);
    count
}
