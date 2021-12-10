use aoc21::day;

fn end_to_start(c: char) -> char {
    match c {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => panic!("invalid char"),
    }
}

fn error_value(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("invalid char"),
    }
}

fn incomplete_value(c: char) -> usize {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => panic!("invalid char"),
    }
}

enum LineStatus {
    Corrupt(usize),
    Incomplete(usize),
}

impl LineStatus {
    fn get_corrupt(&self) -> Option<usize> {
        match self {
            &LineStatus::Corrupt(x) => Some(x),
            _ => None,
        }
    }

    fn get_incomplete(&self) -> Option<usize> {
        match self {
            &LineStatus::Incomplete(x) => Some(x),
            _ => None,
        }
    }
}

fn get_status(line: &str) -> LineStatus {
    let mut stack = vec![];
    for char in line.chars() {
        if ['(', '[', '{', '<'].contains(&char) {
            stack.push(char);
        } else {
            match stack.pop() {
                Some(next_close) if next_close == end_to_start(char) => {}
                _ => { return LineStatus::Corrupt(error_value(char)); }
            }
        }
    }

    let score = stack.into_iter()
        .rev()
        .map(incomplete_value)
        .fold(0, |acc, e| acc * 5 + e);
    LineStatus::Incomplete(score)
}

fn solve(input: String) -> (usize, usize) {
    let statuses = input.lines()
        .map(get_status)
        .collect::<Vec<_>>();

    let part1 = statuses.iter()
        .filter_map(LineStatus::get_corrupt)
        .sum::<usize>();

    let part2 = {
        let mut values = statuses.iter()
            .filter_map(LineStatus::get_incomplete)
            .collect::<Vec<_>>();
        values.sort_unstable();
        values[values.len() / 2]
    };

    (part1, part2)
}

day!(10, solve);
