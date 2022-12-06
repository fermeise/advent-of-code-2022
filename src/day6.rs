use itertools::Itertools;

pub fn solve(input: &str) -> (usize, usize) {
    (solve_part1(input), solve_part2(input))
}

fn solve_part1(input: &str) -> usize {
    (3..input.len())
        .find(|i| input[i - 3..i + 1].chars().all_unique())
        .unwrap()
        + 1
}

fn solve_part2(input: &str) -> usize {
    (13..input.len())
        .find(|i| input[i - 13..i + 1].chars().all_unique())
        .unwrap()
        + 1
}
