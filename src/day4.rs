use std::io;
use std::io::Read;
use std::str::FromStr;

#[derive(Debug)]
struct Board {
    board: [[(u32, bool); 5]; 5],
}

impl FromStr for Board {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers = s.split_whitespace().map(|x| u32::from_str(x).unwrap());
        let mut board = [[(0, false); 5]; 5];
        for r in 0..5 {
            for c in 0..5 {
                board[r][c] = (numbers.next().unwrap(), false);
            }
        }

        Ok(Board { board })
    }
}

impl Board {
    fn mark(&mut self, n: u32) {
        self.board.iter_mut()
            .flatten()
            .filter(|(x, _)| *x == n)
            .for_each(|(_, m)| *m = true);
    }

    fn bingo(&self) -> bool {
        let row_win = self.board.iter()
            .any(|r| r.iter().all(|(_, m)| *m));
        let col_win = (0..5).any(|c| self.board.iter()
            .map(|r| r[c].1)
            .all(|m| m));

        row_win || col_win
    }

    fn sum(&self) -> u32 {
        self.board.iter()
            .flatten()
            .filter(|(_, m)| !*m)
            .map(|(n, _)| *n)
            .sum()
    }
}

fn main() {
    let mut contents = String::new();
    io::stdin().read_to_string(&mut contents).unwrap();
    let (numbers, boards) = contents.split_once("\n\n").unwrap();
    let numbers = numbers.split(',')
        .map(|x| u32::from_str(x).unwrap())
        .collect::<Vec<_>>();
    let mut boards = boards.split("\n\n")
        .map(|x| Board::from_str(x).unwrap())
        .collect::<Vec<_>>();

    let mut first = true;
    for number in numbers {
        boards.iter_mut().for_each(|b| b.mark(number));
        if let Some(winner) = boards.iter().find(|b| b.bingo()) {
            if first {
                let part1 = number * winner.sum();
                println!("Part 1: {}", part1);
                first = false;
            }

            if boards.len() == 1 {
                let part2 = number * winner.sum();
                println!("Part 2: {}", part2);
                break;
            }
            boards.retain(|b| !b.bingo());
        }
    }
}
