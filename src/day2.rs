use std::str::FromStr;
use aoc21::get_input_list;
use crate::Command::{Down, Forward, Up};

enum Command {
    Forward(i64),
    Down(i64),
    Up(i64),
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut oof = s.split(' ');
        let command = oof.next().unwrap();
        let distance = i64::from_str(oof.next().unwrap()).unwrap();

        Ok(match command {
            "forward" => Forward(distance),
            "down" => Down(distance),
            "up" => Up(distance),
            _ => panic!("invalid command")
        })
    }
}

fn main() {
    let commands = get_input_list::<Command>().unwrap();

    let part1 = {
        let mut position = 0;
        let mut depth = 0;
        for command in &commands {
            match command {
                Forward(d) => position += d,
                Down(d) => depth += d,
                Up(d) => depth -= d,
            }
        }

        position * depth
    };

    let part2 = {
        let mut position = 0;
        let mut depth = 0;
        let mut aim = 0;
        for command in commands {
            match command {
                Forward(d) => {
                    position += d;
                    depth += aim * d;
                }
                Down(d) => aim += d,
                Up(d) => aim -= d,
            }
        }

        position * depth
    };

    println!("Part 1: {}\nPart 2: {}", part1, part2);
}
