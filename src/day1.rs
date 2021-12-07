use aoc21::get_input_list;

fn count_increasing(values: &[i64]) -> usize {
    values.windows(2).filter(|l| l[1] > l[0]).count()
}

fn main() {
    let lines = get_input_list::<i64>().unwrap();

    // Part 1
    let part1 = count_increasing(&lines);

    // Part 2
    let windows = lines.windows(3).map(|x| x[0] + x[1] + x[2]).collect::<Vec<_>>();
    let part2 = count_increasing(&windows);

    println!("Part 1: {}\nPart 2: {}", part1, part2);
}
