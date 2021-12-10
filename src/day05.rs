use std::cmp::{max, min};
use std::collections::HashMap;
use std::str::FromStr;
use aoc21::{get_input_list, day};

struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Line {
    fn straight(&self) -> bool {
        self.x1 == self.x2 || self.y1 == self.y2
    }

    fn points(&self, diagonals: bool) -> Vec<(i32, i32)> {
        if self.x1 == self.x2 {
            let start = min(self.y1, self.y2);
            let end = max(self.y1, self.y2);
            (start..=end).map(|y| (self.x1, y)).collect()
        } else if self.y1 == self.y2 {
            let start = min(self.x1, self.x2);
            let end = max(self.x1, self.x2);
            (start..=end).map(|x| (x, self.y1)).collect()
        } else if diagonals {
            let dx = if self.x1 < self.x2 { 1 } else { -1 };
            let dy = if self.y1 < self.y2 { 1 } else { -1 };
            (0..=(self.x1 - self.x2).abs())
                .map(|i| (self.x1 + dx * i, self.y1 + dy * i))
                .collect()
        } else {
            panic!("Invalid state")
        }
    }
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p1, p2) = s.split_once(" -> ").unwrap();
        let (x1, y1) = p1.split_once(',').unwrap();
        let (x2, y2) = p2.split_once(',').unwrap();
        Ok(Line {
            x1: i32::from_str(x1).unwrap(),
            y1: i32::from_str(y1).unwrap(),
            x2: i32::from_str(x2).unwrap(),
            y2: i32::from_str(y2).unwrap(),
        })
    }
}

fn count_duplicates(points: impl Iterator<Item=(i32, i32)>) -> usize {
    points
        .fold(HashMap::new(), |mut m, e| {
            *m.entry(e).or_insert(0) += 1;
            m
        })
        .into_iter()
        .filter(|&(_, y)| y > 1)
        .count()
}

fn solve(input: String) -> (usize, usize) {
    let lines = get_input_list::<Line>(input).unwrap();

    let part1 = count_duplicates(lines.iter()
        .filter(|l| l.straight())
        .flat_map(|l| l.points(false)));

    let part2 = count_duplicates(lines.iter()
        .flat_map(|l| l.points(true)));

    (part1, part2)
}

day!(5, solve);
