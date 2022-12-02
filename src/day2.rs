pub fn solve(input: &str) -> (usize, usize) {
    (solve_part1(input), solve_part2(input))
}

fn solve_part1(input: &str) -> usize {
    let mut total: usize = 0;
    for x in input.lines() {
        if let (Some(them), Some(me)) = scan_fmt_some!(x, "{} {}", String, String) {
            let them = match them.as_ref() {
                "A" => 0,
                "B" => 1,
                "C" => 2,
                _ => panic!(),
            };
            let me = match me.as_ref() {
                "X" => 0,
                "Y" => 1,
                "Z" => 2,
                _ => panic!(),
            };
            let score = match (3 + me - them) % 3 {
                0 => 3,
                1 => 6,
                2 => 0,
                _ => panic!(),
            };
            total += score + me + 1;
        } else {
            panic!()
        }
    }
    total
}

fn solve_part2(input: &str) -> usize {
    let mut total: usize = 0;
    for x in input.lines() {
        if let (Some(them), Some(score)) = scan_fmt_some!(x, "{} {}", String, String) {
            let them = match them.as_ref() {
                "A" => 0,
                "B" => 1,
                "C" => 2,
                _ => panic!(),
            };
            let (score, me) = match score.as_ref() {
                "X" => (0, (3 + them - 1) % 3),
                "Y" => (3, them),
                "Z" => (6, (them + 1) % 3),
                _ => panic!(),
            };
            total += score + me + 1;
        } else {
            panic!()
        }
    }
    total
}
