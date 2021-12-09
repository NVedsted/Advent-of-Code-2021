use std::collections::BinaryHeap;

use aoc21::{breadth_first_search, day, get_input_list};

fn get_surrounding(r: usize, c: usize, height: usize, width: usize) -> Vec<(usize, usize)> {
    let mut surrounding = vec![];
    if r > 0 {
        surrounding.push((r - 1, c));
    }
    if r < height - 1 {
        surrounding.push((r + 1, c));
    }
    if c > 0 {
        surrounding.push((r, c - 1));
    }
    if c < width - 1 {
        surrounding.push((r, c + 1));
    }
    surrounding
}

fn solve(input: String) -> (usize, usize) {
    let map = get_input_list::<String>(input).unwrap()
        .into_iter()
        .map(|r| r.chars()
            .map(|c| c as usize - '0' as usize)
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let height = map.len();
    let width = map[0].len();

    let low_points = (0..height)
        .flat_map(|r| (0..width).map(move |c| (r, c)))
        .filter(|&(r, c)| get_surrounding(r, c, height, width).into_iter()
            .all(|(sr, sc)| map[sr][sc] > map[r][c]))
        .collect::<Vec<_>>();

    let part1 = low_points.iter()
        .cloned()
        .map(|(r, c)| map[r][c] + 1)
        .sum::<usize>();

    let mut basin_sizes = low_points.iter()
        .map(|&low_point| {
            breadth_first_search(low_point, |(r, c), queue| {
                get_surrounding(r, c, height, width).into_iter()
                    .filter(|&(r,c)| map[r][c] != 9)
                    .for_each(|e| queue.push_back(e));
            }).len()
        })
        .collect::<BinaryHeap<_>>();

    let part2 = (0..3)
        .map(|_| basin_sizes.pop().unwrap())
        .product();

    (part1, part2)
}

day!(9, solve);
