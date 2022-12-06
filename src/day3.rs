use std::collections::HashSet;

use itertools::Itertools;

pub fn solve(input: &str) -> (u32, u32) {
    (solve_part1(input), solve_part2(input))
}

fn solve_part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let items: Vec<_> = line.chars().collect();
            let first = &items[0..items.len() / 2];
            let second = &items[items.len() / 2..items.len()];
            let found: HashSet<_> = first.iter().collect();
            let item = second.iter().find(|item| found.contains(item)).unwrap();
            priority(*item)
        })
        .sum()
}

fn solve_part2(input: &str) -> u32 {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|elves| {
            let badge = elves
                .map(|items| items.chars().collect::<HashSet<char>>())
                .reduce(|accum, item| accum.intersection(&item).cloned().collect())
                .unwrap();
            priority(*badge.iter().next().unwrap())
        })
        .sum()
}

fn priority(c: char) -> u32 {
    if c >= 'a' && c <= 'z' {
        c as u32 - 'a' as u32 + 1
    } else if c >= 'A' && c <= 'Z' {
        c as u32 - 'A' as u32 + 27
    } else {
        panic!()
    }
}
