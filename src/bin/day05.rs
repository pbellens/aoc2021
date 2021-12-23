use anyhow::Result;
use std::{collections::HashMap, ops::RangeInclusive, str::FromStr};


#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point(u32, u32);

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(",").unwrap();
        Ok(Point(x.parse()?, y.parse()?))
        
    }
}


#[derive(Debug)]
pub struct Line {
    start: Point,
    end: Point,
}

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (left, right) = s.split_once(" -> ").unwrap();

        Ok(Line {
            start: left.parse()?,
            end: right.parse()?,
        })
    }
}


fn steps(start: u32, end: u32) -> Box<dyn Iterator<Item = u32>> {
    if start < end {
        Box::new(RangeInclusive::new(start, end))
    } else {
        Box::new(RangeInclusive::new(end, start).rev())
    }
}

fn between(start: Point, end: Point, diag: bool) -> Vec<Point> {
    if start.0 != end.0 && start.1 != end.1 {
        if !diag {
            Vec::new()
        } else {
            steps(start.0, end.0)
                .zip(steps(start.1, end.1))
                .map(|(x, y)| Point(x, y))
                .collect()
        }
    } else {
        steps(start.0, end.0)
            .flat_map(|x| steps(start.1, end.1).map(move |y| Point(x, y)))
            .collect()
    }
}

fn find_overlaps(lines: &Vec<Line>, diag: bool) -> usize {
    lines.iter()
        .fold(
            HashMap::new(),
            |mut acc, l| {
                between(l.start, l.end, diag)
                    .into_iter()
                    .for_each(|p| { acc.insert(p, acc.get(&p).unwrap_or(&0) + 1); () }); 
                acc 
            })
        .into_iter()
        .fold(
            0,
            |acc, (_p, c)| if c > 1 { acc + 1 } else { acc })
}


fn main() -> Result<()> {
    let lines: Vec<Line> = util::read_one_per_line("./data/day05.input")?;  

    println!("Solution part 1: {}", find_overlaps(&lines, false));
    println!("Solution part 2: {}", find_overlaps(&lines, true));

    Ok(())
}
