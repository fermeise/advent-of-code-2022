pub fn solve(input: &str) -> (usize, usize) {
    (solve_part1(input), solve_part2(input))
}

fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (a_from, a_to, b_from, b_to) =
                scan_fmt!(line, "{}-{},{}-{}", u32, u32, u32, u32).unwrap();
            if a_from <= b_from && b_to <= a_to {
                1
            } else if b_from <= a_from && a_to <= b_to {
                1
            } else {
                0
            }
        })
        .sum()
}

fn solve_part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (a_from, a_to, b_from, b_to) =
                scan_fmt!(line, "{}-{},{}-{}", u32, u32, u32, u32).unwrap();
            if a_from <= b_from && a_to >= b_from {
                1
            } else if b_from <= a_from && b_to >= a_from {
                1
            } else {
                0
            }
        })
        .sum()
}
