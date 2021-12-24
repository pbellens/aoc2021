use anyhow::Result;
use std::array::IntoIter;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;

struct ErrorScore {
    stack: Vec<char>,
    errorc: usize,
}

impl ErrorScore {
    fn new() -> ErrorScore {
        ErrorScore { stack: Vec::new(), errorc: 0 }
    }
}

fn main() -> Result<()> {
    let open: HashSet<char> = HashSet::from_iter(IntoIter::new(['(', '[', '{', '<']));
    let close: HashSet<char> = HashSet::from_iter(IntoIter::new([')', ']', '}', '>']));
    let o2c = HashMap::<char, char>::from_iter(IntoIter::new([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]));
    let c2o = HashMap::<char, char>::from_iter(IntoIter::new([(')', '('), (']', '['), ('}', '{'), ('>', '<')]));

    let score = include_str!("../../data/day10.input")
        .lines()
        .fold(
            ErrorScore::new(),
            |mut score, l| {
                for c in l.chars() {
                    if open.contains(&c) {
                        score.stack.push(c);
                    } else {
                        let openbracket = score.stack.last().unwrap();
                        let exp = c2o.get(&c).unwrap();

                        if openbracket != exp {
                            score.errorc += match c {
                                ')' => 3,
                                ']' => 57,
                                '}' => 1197,
                                '>' =>  25137,
                                _ => panic!("Unbrackety bracket!")
                            };
                            return score;
                        } else {
                            score.stack.pop();
                        }
                    }
                }
                score
            });
    println!("Solution part 1: {}", score.errorc);

     let sortscores = include_str!("../../data/day10.input")
        .lines()
        .fold(
            Vec::new(),
            |mut scores, l| {
                let mut stack: Vec<char> = Vec::new();
                for c in l.chars() {
                    if open.contains(&c) {
                        stack.push(c);
                    } else {
                        let openbracket = stack.last().unwrap();
                        let exp = c2o.get(&c).unwrap();
                        if openbracket != exp {
                            return scores;
                        } else {
                            stack.pop();
                        }
                    }
                }

                println!("leftover {}", stack.len());
                let score : usize = stack.into_iter().rev().fold(
                    0, 
                    |acc, c| match o2c.get(&c).unwrap() {
                        ')' => acc * 5 + 1,
                        ']' => acc * 5 + 2,
                        '}' => acc * 5 + 3,
                        '>' => acc * 5 + 4,
                        _ => panic!("Unbrackety bracket!")
                    });
                if score > 0 {
                    scores.push(score);
                }

                scores
            })
        .into_iter()
        .sorted()
        .collect_vec();

        println!("found {}", sortscores.len());
        println!("Solution part 2: {:?}", sortscores[sortscores.len()/2]);


    Ok(())
}
