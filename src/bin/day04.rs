use anyhow::Result;
use std::collections::HashSet;
use itertools::Itertools;
use std::str::Lines;


#[derive(Debug, Clone)]
struct BingoBoard {
    id: u32,
    rows: Vec<HashSet<u32>>,
    cols: Vec<HashSet<u32>>,
    bingo: bool,
}

impl BingoBoard {
      fn new(lines: &mut Lines, dim: usize, id: u32) -> Option<BingoBoard> {
        let mut sets: Vec<HashSet<u32>> = Vec::new();

        let empty = lines.next()?;
        if empty != "" {
            return None; // Hmmm. 
        }

        let vrows: Vec<Vec<u32>> = lines
            .take(dim)
            .map(|l| l.split_whitespace().map(|m| m.parse().unwrap()).collect())
            .collect_vec();

        let mut cols = vec![];
        for col in 0..dim {
            let mut c = HashSet::new();
            for row in 0..dim {
                c.insert(vrows[row][col]);
            }

            cols.push(c);
        }

        // Have to do this last because it consumes vrows
        let rows = vrows.into_iter().map(|r| HashSet::from_iter(r)).collect_vec();

        Some(BingoBoard { id: id, rows: rows, cols: cols, bingo: false })
    }

    fn stamp(&mut self, number: u32) -> bool {
        let rdone = self.rows.iter_mut().fold(
            false, 
            |acc, r| { r.remove(&number); acc | r.is_empty() });
        let cdone = self.cols.iter_mut().fold(
            false, 
            |acc, r| { r.remove(&number); acc | r.is_empty() });
        
        self.bingo = rdone | cdone;
        self.bingo
    }

    fn leftover(&self) -> u32 {
        self.rows.iter().fold(0, |acc, v| acc + v.iter().sum::<u32>())
    }
}


fn main() -> Result<()> {
    let dim: usize = 5;

    let mut lines = include_str!("../../data/day04.input").lines();
    let moves = lines
        .next()
        .unwrap()
        .split(",")
        .map(|m| m.parse::<u32>().unwrap())
        .collect_vec();

    let mut boards = vec![];
    let mut id: u32 = 0;
    while let Some(board) = BingoBoard::new(&mut lines, dim, id) {
        boards.push(board);
        id += 1;
    }

    let mut boards1 = boards.clone();
    'outer1: for m in &moves {
        for board in boards1.iter_mut() {
            if board.stamp(*m) {
                println!("Solution part 1: {}", m * board.leftover());
                break 'outer1;
            }
        }
    }

    let mut boards2 = boards.clone();
    let mut left = boards.len();
    'outer2: for m in &moves {
        for board in boards2.iter_mut().filter(|b| !b.bingo) {
            if board.stamp(*m) {
                left -= 1;
                if left == 0 {
                    println!("Solution part 2: {}", m * board.leftover());
                    break 'outer2;
                }
            }
        }
    }

    Ok(())
}
