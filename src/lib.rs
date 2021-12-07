use std::io;
use std::io::BufRead;
use std::str::FromStr;

pub fn get_input_lines() -> Vec<String> {
    io::stdin().lock().lines().map(|l| l.unwrap()).collect::<Vec<String>>()
}

pub fn get_input_list<T: FromStr>() -> Result<Vec<T>, T::Err> {
    get_input_lines().into_iter()
        .filter(|s| s.len() > 0)
        .map(|s| T::from_str(&s))
        .collect::<Result<Vec<_>, _>>()
}
