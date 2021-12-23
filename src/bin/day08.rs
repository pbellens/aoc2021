use anyhow::{anyhow, Result};
use std::str::FromStr;
use itertools::Itertools;

enum Segment {
    Top,                // a
    TopLeft,            // b
    TopRight,           // c
    Middle,             // d
    BottomLeft,         // e
    BottomRight,        // f
    Bottom,             // g
}

impl FromStr for Segment {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" => Result::Ok(Segment::Top),
            "b" => Result::Ok(Segment::TopLeft),
            "c" => Result::Ok(Segment::TopRight),
            "d" => Result::Ok(Segment::Middle),
            "e" => Result::Ok(Segment::BottomLeft),
            "f" => Result::Ok(Segment::BottomRight),
            "g" => Result::Ok(Segment::Bottom),
            _   => Result::Err(anyhow!("Letter does not correspond to segment"))
        }
    }
}

fn main() -> Result<()> {
    println!("Solution part 1: {}", 
        include_str!("../../data/day08.input")
            .lines()
                    .map(|l| l.split_once(" | ").unwrap().1)
                .flat_map(|ss| ss.split_whitespace().collect_vec())
            .filter(|displ| match displ.len() {
                2 | 3 | 4 | 7 => true,
                _ => false
            })
            .count());

    Ok(())
}



