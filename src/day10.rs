use aoc21::day;

fn get_start_from_end(c: char) -> char {
    match c {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => panic!("invalid char"),
    }
}

fn get_error_value(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("invalid char"),
    }
}

fn get_incomplete_value(c: char) -> usize {
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
    for current in line.chars() {
        if ['(', '[', '{', '<'].contains(&current) {
            stack.push(current);
        } else {
            match stack.pop() {
                Some(next_close) if next_close == get_start_from_end(current) => {}
                _ => { return LineStatus::Corrupt(get_error_value(current)); }
            }
        }
    }

    let score = stack.into_iter()
        .map(get_incomplete_value)
        .rfold(0, |acc, e| acc * 5 + e);
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
        let middle = values.len() / 2;
        values.select_nth_unstable(middle).1.clone()
    };

    (part1, part2)
}

day!(10, solve);
