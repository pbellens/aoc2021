use anyhow::Result;
use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;

#[derive(Debug)]
enum Direction {
    Up(u64), 
    Down(u64) ,
    Forward(u64),
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if let Some((direction, size)) = s.split_once(" ") {
            let s = size.parse::<u64>()?;
            let d = match direction {
                "up" => Direction::Up(s), 
                "down" => Direction::Down(s), 
                "forward" => Direction::Forward(s) ,
                _ => panic!("Unknown direction")
            };
            return Ok(d)
        }
        Err(anyhow::format_err!("Could not find direction and size"))
    }
}

#[derive(Debug)]
struct Pos {
    depth: u64,
    position: u64,
    aim: u64
}

impl Pos {
    fn new() -> Pos {
        Pos {
            depth: 0,
            position: 0,
            aim: 0
        }
    }

    fn multiply(&self) -> u64 {
        self.depth * self.position
    }
}

fn main() -> Result<()> {
    println!(
        "Solution for part 1: {:?}", 
        util::read_one_per_line::<Direction>("data/day02.input")?
            .iter()
            .fold(
                Pos::new(), 
                |acc, dir| match dir {
                    Direction::Up(u) => Pos { depth: acc.depth - u, ..acc },
                    Direction::Down(d) => Pos { depth: acc.depth + d, ..acc },
                    Direction::Forward(f) => Pos { position: acc.position + f, .. acc}
                })
            .multiply());
    println!(
        "Solution for part 2: {:?}", 
        util::read_one_per_line::<Direction>("data/day02.input")?
            .iter()
            .fold(
                Pos::new(), 
                |acc, dir| match dir {
                    Direction::Up(u) => Pos { aim: acc.aim - u, ..acc },
                    Direction::Down(d) => Pos { aim: acc.aim + d, ..acc },
                    Direction::Forward(f) => Pos { position: acc.position + f, depth: acc.depth + acc.aim * f, ..acc }
                })
            .multiply());

    return Ok(())
}
