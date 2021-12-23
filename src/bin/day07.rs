use anyhow::Result;
use std::ops::RangeInclusive;

fn minimize<F>(crabs: &Vec<i64>, cost: F, range: RangeInclusive<i64>) -> i64 
    where F: Fn(i64, i64) -> i64 
{
    let mut bestcost = i64::MAX;
    for cand in range {
        let total = crabs.iter()
            .fold(0, |acc, c| acc + cost(cand, *c));

        if total < bestcost {
            bestcost = total; 
        } 
    }

    bestcost
}

fn main() -> Result<()> {
    let crabs: Vec<i64> = util::read_one_line("./data/day07.input", ",")?;
    let r = *crabs.iter().min().unwrap()..=*crabs.iter().max().unwrap();

    println!("Solution part 1: {}", minimize(&crabs, |a, b| i64::abs(a - b), r.clone()));
    println!("Solution part 2: {}", minimize(&crabs, |a, b| { let d = i64::abs(a - b); return d * (d + 1) / 2 }, r.clone()));

    Ok(())
}





