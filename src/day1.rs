pub fn solve(input: &str) -> (i32, i32) {
    (solve_part1(input), solve_part2(input))
}

fn solve_part1(input: &str) -> i32 {
    let lines: Vec<_> = input.lines().map(|x| x.parse::<i32>()).collect();
    let elves = lines.split(|num| num.is_err());
    elves
        .map(|lines| {
            lines
                .iter()
                .map(|calories| calories.as_ref().unwrap())
                .sum()
        })
        .max()
        .unwrap()
}

fn solve_part2(input: &str) -> i32 {
    let lines: Vec<_> = input.lines().map(|x| x.parse::<i32>()).collect();
    let elves = lines.split(|num| num.is_err());
    let mut totals: Vec<i32> = elves
        .map(|lines| {
            lines
                .iter()
                .map(|calories| calories.as_ref().unwrap())
                .sum()
        })
        .collect();
    totals.sort();
    totals.reverse();
    totals[0] + totals[1] + totals[2]
}
