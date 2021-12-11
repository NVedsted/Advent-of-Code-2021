use std::{fs, io};
use std::collections::{HashSet, VecDeque};
use std::hash::Hash;
use std::io::Read;
use std::str::FromStr;
use std::time::Instant;

pub fn get_input_list<T: FromStr>(input: String) -> Result<Vec<T>, T::Err> {
    input.trim().split('\n').into_iter()
        .filter(|s| s.trim().len() > 0)
        .map(|s| T::from_str(&s))
        .collect::<Result<Vec<_>, _>>()
}

pub fn get_input_csv<T: FromStr>(input: String) -> Result<Vec<T>, T::Err> {
    input.trim().split(',')
        .filter(|s| s.trim().len() > 0)
        .map(|s| T::from_str(s.trim()))
        .collect::<Result<Vec<_>, _>>()
}

pub fn breadth_first_search<T, F>(start: T, mut visit: F) -> HashSet<T>
    where
        T: Hash + Eq + Clone,
        F: FnMut(T, &mut VecDeque<T>) {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(start);
    while let Some(e) = queue.pop_front() {
        if visited.contains(&e) {
            continue;
        }
        visited.insert(e.clone());
        visit(e, &mut queue);
    }

    visited
}

#[doc(hidden)]
pub fn day_run<A: ToString, B: ToString>(day: usize, solve: fn(String) -> (A, B), test: bool) {
    let input_extension = if test { "test-in" } else { "in" };
    let output_extension = if test { "test-out" } else { "out" };

    let input = if let Ok(input) = fs::read_to_string(format!(".data/{}.{}", day, input_extension)) {
        println!("Using input in .data/{}.{}", day, input_extension);
        input
    } else {
        println!("Reading input from STDIN.");
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        input
    };

    let (part1, part2) = {
        let start = Instant::now();
        let (part1, part2) = solve(input);
        let elapsed = start.elapsed();
        println!("Solve took {:?}", elapsed);
        (part1.to_string(), part2.to_string())
    };
    println!("Part 1: {}\nPart 2: {}", part1, part2);
    if let Ok(output) = fs::read_to_string(format!(".data/{}.{}", day, output_extension)) {
        let cases = output.trim().split('\n')
            .map(str::trim)
            .collect::<Vec<_>>();
        if cases.len() > 0 {
            assert_eq!(part1.to_string(), cases[0]);
            println!("Part 1 correct according to .data/{}.{}", day, output_extension);
        } else {
            println!("Part 1 not verified.");
        }
        if cases.len() > 1 {
            assert_eq!(part2.to_string(), cases[1]);
            println!("Part 2 correct according to .data/{}.{}", day, output_extension);
        } else {
            println!("Part 2 not verified.");
        }
    } else {
        println!("Create .data/{}.{} to verify solutions", day, output_extension);
    }
}

#[macro_export]
macro_rules! day {
    ($day:literal, $solve:ident)=>{
        fn main() { aoc21::day_run($day, $solve, false) }
        #[cfg(test)] mod tests {
            use crate::*;
            #[test] fn test_day() { aoc21::day_run($day, $solve, true); }
        }
    }
}
