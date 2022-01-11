use itertools::Itertools;
use anyhow::Result;
use std::collections::HashSet;

struct School {
    squids: Vec<u32>,
    m: usize, 
    n: usize,
}

impl School {
    fn new(squids: Vec<u32>, m: usize, n: usize) -> Self {
        School { squids, m, n }
    }

    fn flash(mut self, x: usize, y: usize) -> Self {
        self.squids[x * self.m + y] += 1;
        self
    }

    fn flash_all(mut self) -> Self {
        //self.squids.into_iter().for_each(|s| s += 1);
        self
    }
}

fn neighbors(x: usize, y: usize, m: usize, n: usize) -> Vec<(usize, usize)> {
    let sx: i32 = x.try_into().unwrap();
    let sy: i32 = x.try_into().unwrap();
    let sm: i32 = m.try_into().unwrap();
    let sn: i32 = n.try_into().unwrap();

    vec![(-1,-1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1) ]
        .into_iter()
        .map(|(i, j)| (sx + i, sx + j))
        .filter(|&(x,y)| x >= 0 && x <= sm && y >= 0 && y <= sn)
        .map(|(x, y)| (x.try_into().unwrap(), y.try_into().unwrap()))
        .collect_vec()
}

fn flashes(mut school: School, x: usize, y: usize, flashed: HashSet<(usize, usize)>) -> (HashSet<(usize, usize)>, School) {
    let p1 = neighbors(x, y, school.m, school.n)
        .into_iter()
        .filter(|n| !flashed.contains(&n))
        .filter(|n| school.squids[school.m * n.0 + n.1] == 9)
        .collect_vec();
    p1.into_iter()
        .fold(
            (flashed, school),
            |mut acc, n| { 
                let school = acc.1.flash(n.0, n.1);
                acc.0.insert((n.0, n.1));
                flashes(school, n.0, n.1, acc.0)
            })
}

fn step(mut school: School) -> School {
    let mut flashed = HashSet::new();

    // bump all squids
    school = school.flash_all();

    for x in 0..school.m {
        for y in 0..school.n {
            if school.squids[school.m * x + y] > 9 {
                flashed.insert((x, y));
            }
        }
    }

    for (x, y) in flashed.clone().into_iter() {
        let result = flashes(school, x, y, flashed);
        flashed = result.0;
        school = result.1;
    }

    for (x, y) in flashed.iter() {
        school.squids[school.m * *x + *y] = 0;
    }

    //(school, flashed.len())
    school
}

fn main() -> Result<()> {
    const M: usize = 10;
    const N: usize = 10;

    let mut school = School::new(
        include_str!("../../data/day11.input")
            .lines()
            .flat_map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
            .collect_vec(),
        M, N);

    school = step(school);
        
    Ok(())
}
