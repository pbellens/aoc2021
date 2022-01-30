use anyhow::{Error, Result};
use std::str::FromStr;
use std::collections::HashSet;
use std::cmp::Ordering;


#[derive(Debug, Clone, Copy)]
enum Fold {
    X(usize), 
    Y(usize)
}
#[derive(Debug, Clone, Copy)]
struct FoldParseError;

impl FromStr for Fold {
    type Err = FoldParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once('=') {
            Some(("fold along x", x)) => Ok(Fold::X(x.parse().unwrap())),
            Some(("fold along y", y)) => Ok(Fold::Y(y.parse().unwrap())),
            Some((&_, _)) => unreachable!()
 ,           None => Err(FoldParseError{})
        }
    }
}


#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: usize, 
    y: usize
}
#[derive(Debug, Clone, Copy)]
struct PointParseError;

impl FromStr for Point {
    type Err = PointParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(',') {
            Some((x, y)) => Ok(Point { 
                x: x.parse().unwrap(), 
                y: y.parse().unwrap()}),
            None => Err(PointParseError{})
        }
    }
}


fn main() -> Result<()>
{
    let (points, folds) = include_str!("../../data/day13.input")
        .trim()
        .split_once("\n\n")
        .unwrap();

    let points: HashSet<Point> = points.lines().map(str::parse).map(Result::unwrap).collect();
    let folds: Vec<Fold> = folds.lines().map(str::parse).map(Result::unwrap).collect();

    let res1 = folds.iter().take(1).fold(
        points.clone(), 
        |acc, f| {
            acc.iter()
                .map(|p| 
                    match f {
                        Fold::X(x) => if p.x < *x { *p } else { Point {x: *x - (p.x - *x), y: p.y } },
                        Fold::Y(y) => if p.y < *y { *p } else { Point {x: p.x , y: *y - (p.y - *y) } },
                    })
                .collect()
        });

    println!("Solution day 1: {:?}", res1.len());

    let res2 = folds.iter().fold(
        points.clone(), 
        |acc, f| {
            acc.iter()
                .map(|p| 
                    match f {
                        Fold::X(x) => if p.x < *x { *p } else { Point {x: *x - (p.x - *x), y: p.y } },
                        Fold::Y(y) => if p.y < *y { *p } else { Point {x: p.x , y: *y - (p.y - *y) } },
                    })
                .collect()
        });

    let map = res2.iter()
        .fold(
            vec![' '; 50 * 8],
            |mut acc, p| {
                acc[p.y * 40 + p.x] = 'X';
                acc
            });

    for y in 0..6 {
        for x in 0..40 {
            print!("{}", map[y * 40 + x])
        }
        println!("");
    }

    Ok(())
}

