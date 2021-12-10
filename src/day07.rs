use aoc21::{get_input_csv, day};

fn solve(input: String) -> (i32, i32) {
    let mut positions = get_input_csv::<i32>(input).unwrap();
    positions.sort_unstable();

    let median = positions[positions.len() / 2];
    let part1 = positions.iter().map(|x| (x - median).abs())
        .sum::<i32>();

    let smallest = positions.iter().min().unwrap().clone();
    let biggest = positions.iter().max().unwrap().clone();
    let part2 = (smallest..=biggest)
        .map(|x| positions.iter()
            .map(|c| {
                let d = (c - x).abs();
                d * (d + 1) / 2
            })
            .sum::<i32>())
        .min().unwrap();

    (part1, part2)
}

day!(7, solve);
