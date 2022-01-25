use anyhow::{Error, Result};
use std::str::FromStr;


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
            None => Err(FoldParseError{})
        }
    }
}


#[derive(Debug, Clone, Copy)]
struct Point {
    x: usize, 
    y: usize
}
#[derive(Debug, Clone, Copy)]
struct PointParseError;

impl FromStr for Point {
    type Err = MyError;

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
    let (points, folds) = include_str!("../../data/day12.input")
        .trim()
        .split_once("\n\n'")
        .unwrap();

    let points: Vec<Point> = points.lines().map(str::parse).map(Result::unwrap).collect();
    let folds: Vec<Fold> = folds.lines().map(str::parse).map(Result::unwrap).collect();

    Ok(())
}

