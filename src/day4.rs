use std::{cell::Cell, fs, ops::Index, path::Path};

use crate::util;

pub fn part1() {
    let filename = "./input/day4.txt";
    let bingo_game = read_bingo(filename);

    for n in bingo_game.numbers {
        let mut winning_scores: Vec<i32> = Vec::new();
        for b in &bingo_game.boards {
            let won = b.receive_number(n);
            if won {
                let score = b.calculate_score(n);
                winning_scores.push(score);
            }
        }

        // It isn't stated in the problem, but there might be ties; find the highest score
        if winning_scores.len() > 0 {
            winning_scores.sort();
            let result = *winning_scores.last().unwrap();
            println!("day4part1: {}", result);
            return;
        }
    }

    let result = "no winning board";

    println!("day4part1: {}", result);
}

pub fn part2() {
    let filename = "./input/day4.txt";

    let result = "unimpl";

    println!("day4part2: {}", result);
}

// Split contents of input file by double newline. First match is the list of
// numbers, remaining matches are the bingo boards.
fn read_bingo<P>(filename: P) -> BingoGame
where
    P: AsRef<Path>,
{
    let contents = fs::read_to_string(filename).unwrap();
    let splits: Vec<&str> = contents.split("\n\n").collect();

    let numbers: Vec<i32> = splits
        .index(0)
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let mut boards = Vec::new();
    for i in 1..splits.len() {
        let number_grid: Vec<Vec<i32>> = splits
            .index(i)
            .lines()
            .map(|row| {
                row.split_whitespace()
                    .map(|col| col.parse::<i32>().unwrap())
                    .collect()
            })
            .collect();

        let height = number_grid.len();
        let width = number_grid.index(0).len();
        let marked_grid = vec![vec![Cell::new(false); width]; height];

        let mut board = BingoBoard {
            number_grid,
            marked_grid,
        };
        boards.push(board)
    }

    BingoGame { numbers, boards }
}

struct BingoGame {
    pub numbers: Vec<i32>,
    pub boards: Vec<BingoBoard>,
}

struct BingoBoard {
    pub number_grid: Vec<Vec<i32>>,
    pub marked_grid: Vec<Vec<Cell<bool>>>,
}

impl BingoBoard {
    // FIXME: Returns immediately on a calculated win, even if there are duplicates needing to be marked
    fn receive_number(&self, number: i32) -> bool {
        for y in 0..self.number_grid.len() {
            for x in 0..self.number_grid.index(y).len() {
                if *self.number_grid.index(y).index(x) == number {
                    self.marked_grid.index(y).index(x).set(true);
                    if self.check_won(x, y) {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn check_won(&self, x: usize, y: usize) -> bool {
        self.marked_grid.index(y).iter().all(|col| col.get())
            || self.marked_grid.iter().all(|row| row.index(x).get())
    }

    fn calculate_score(&self, winning_num: i32) -> i32 {
        let mut unmarked_total = 0;
        for y in 0..self.number_grid.len() {
            for x in 0..self.number_grid.index(y).len() {
                if !self.marked_grid.index(y).index(x).get() {
                    unmarked_total += self.number_grid.index(y).index(x);
                }
            }
        }
        return winning_num * unmarked_total;
    }
}
