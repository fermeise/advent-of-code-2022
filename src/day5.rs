use std::collections::VecDeque;
use std::str::Lines;

pub fn solve(input: &str) -> (String, String) {
    (solve_part1(input), solve_part2(input))
}

fn solve_part1(input: &str) -> String {
    let mut lines = input.lines();

    let mut stacks = parse(&mut lines);

    while let Some(line) = lines.next() {
        let (count, from, to) =
            scan_fmt!(line, "move {} from {} to {}", usize, usize, usize).unwrap();
        for _ in 0..count {
            let x = stacks[from - 1].pop_back().unwrap();
            stacks[to - 1].push_back(x);
        }
    }

    stacks.iter().map(|stack| stack.back().unwrap()).collect()
}

fn solve_part2(input: &str) -> String {
    let mut lines = input.lines();

    let mut stacks = parse(&mut lines);

    while let Some(line) = lines.next() {
        let (count, from, to) =
            scan_fmt!(line, "move {} from {} to {}", usize, usize, usize).unwrap();
        let mut tmp = Vec::new();
        for _ in 0..count {
            tmp.push(stacks[from - 1].pop_back().unwrap());
        }
        for _ in 0..count {
            stacks[to - 1].push_back(tmp.pop().unwrap());
        }
    }

    stacks.iter().map(|stack| stack.back().unwrap()).collect()
}

fn parse(lines: &mut Lines) -> Vec<VecDeque<char>> {
    let mut stacks = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        for i in 0..(line.len() + 1) / 4 {
            let chunk = &line[i * 4..i * 4 + 3];
            if let Ok(x) = scan_fmt!(chunk, "[{}]", char) {
                while stacks.len() < i + 1 {
                    stacks.push(VecDeque::new());
                }
                stacks[i].push_front(x);
            }
        }
    }
    stacks
}
