use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use aoc21::day;

// Segments
const A: usize = 1 << 0;
const B: usize = 1 << 1;
const C: usize = 1 << 2;
const D: usize = 1 << 3;
const E: usize = 1 << 4;
const F: usize = 1 << 5;
const G: usize = 1 << 6;

const DIGITS: [usize; 10] = [
    A | B | C | E | F | G, // 0
    C | F, // 1
    A | C | D | E | G, // 2
    A | C | D | F | G, // 3
    B | C | D | F, // 4
    A | B | D | F | G, // 5
    A | B | D | E | F | G, // 6
    A | C | F, // 7
    A | B | C | D | E | F | G, // 8
    A | B | C | D | F | G, // 9
];

fn segment_to_value(segments: usize) -> usize {
    DIGITS.iter()
        .enumerate()
        .find(|&(_, d)| *d == segments)
        .unwrap().0
}

fn solve(input: String) -> (usize, usize) {
    let case_digits = input.lines()
        .map(|l| {
            let (cycle, digits) = l.split_once(" | ").unwrap();
            let cycle = cycle.split_whitespace()
                .map(|d| HashSet::from_iter(d.chars()))
                .collect::<Vec<_>>();
            let digits = digits.split_whitespace()
                .map(|d| HashSet::from_iter(d.chars()))
                .collect::<Vec<_>>();
            get_digits(cycle, digits)
        })
        .collect::<Vec<_>>();

    let unique_lengths = [1, 4, 7, 8].iter()
        .cloned().collect::<HashSet<_>>();
    let part1 = case_digits.iter()
        .flatten()
        .filter(|&d| unique_lengths.contains(d))
        .count();

    let part2 = case_digits.into_iter()
        .map(|digits| digits.into_iter()
            .fold(0, |acc, v| acc * 10 + v)
        )
        .sum::<usize>();

    (part1, part2)
}

fn get_digits(cycle: Vec<HashSet<char>>, digits: Vec<HashSet<char>>) -> Vec<usize> {
    let mapping = compute_mapping(cycle);
    digits.into_iter()
        .map(|digit| segment_to_value(digit.into_iter()
            .map(|e| mapping.get(&e).unwrap())
            .fold(0, |acc, e| acc | e))
        )
        .collect()
}

// The not so pretty HashSet computations...

fn get_intersection(mut sets: impl Iterator<Item=HashSet<char>>) -> HashSet<char> {
    let head = sets.next().unwrap();
    sets.fold(head, |a, b| &a & &b)
}

fn get_element(set: &HashSet<char>) -> char {
    set.iter().next().unwrap().clone()
}

fn compute_mapping(cycle: Vec<HashSet<char>>) -> HashMap<char, usize> {
    let one = cycle.iter().find(|e| e.len() == 2).unwrap();
    let four = cycle.iter().find(|e| e.len() == 4).unwrap();
    let seven = cycle.iter().find(|e| e.len() == 3).unwrap();
    let eight = cycle.iter().find(|e| e.len() == 7).unwrap();
    let group_5_intersection = get_intersection(cycle.iter().cloned().filter(|e| e.len() == 5));
    let group_6_intersection = get_intersection(cycle.iter().cloned().filter(|e| e.len() == 6));

    let a = seven - one;
    let d_g = &group_5_intersection - &a;
    let b_d = four - one;
    let d = &d_g & &b_d;
    let b = &b_d - &d;
    let g = &d_g - &d;
    let f = &(&(&group_6_intersection - &a) - &b) - &g;
    let c = one - &f;
    let e = &(&(&(&(&(eight - &a) - &b) - &c) - &d) - &f) - &g;
    let mut mapping = HashMap::new();
    mapping.insert(get_element(&a), A);
    mapping.insert(get_element(&b), B);
    mapping.insert(get_element(&c), C);
    mapping.insert(get_element(&d), D);
    mapping.insert(get_element(&e), E);
    mapping.insert(get_element(&f), F);
    mapping.insert(get_element(&g), G);
    mapping
}

day!(8, solve);
