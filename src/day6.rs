use std::io;
use std::str::FromStr;

type State = [usize; 9];

fn simulate(start_state: State, days: usize) -> State {
    (0..days).fold(start_state, |prev, _|
        [prev[1], prev[2], prev[3], prev[4], prev[5], prev[6], prev[7] + prev[0], prev[8], prev[0]])
}

fn main() {
    let initial_state = {
        let mut initial_state: State = Default::default();
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        line.trim()
            .split(',')
            .map(|d| usize::from_str(d).unwrap())
            .for_each(|d| initial_state[d] += 1);
        initial_state
    };

    let state_80 = simulate(initial_state, 80);
    let state_256 = simulate(state_80, 256 - 80);

    let part1 = state_80.iter().sum::<usize>();
    let part2 = state_256.iter().sum::<usize>();

    println!("Part 1: {}\nPart 2: {}", part1, part2);
}