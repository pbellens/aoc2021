use anyhow::Result;
use std::str::FromStr;


pub fn read_one_per_line<T: FromStr>(path: &str) -> Result<Vec<T>> {
    Ok(std::fs::read_to_string(path)?
        .lines()
        .filter_map(|line| line.parse::<T>().ok())
        .collect())
}

pub fn read_one_line<T>(path: &str, sep: &str) -> Result<Vec<T>>
where
    T: FromStr,
{
    Ok(std::fs::read_to_string(path)?
        .trim()
        .split(sep)
        .filter_map(|c| c.parse::<T>().ok())
        .collect())
}

pub fn make_size(p: (i32, i32)) -> (usize, usize) {
    (p.0.try_into().unwrap(), p.1.try_into().unwrap())
}

pub fn make_idx(p: (usize, usize)) -> (i32, i32) {
    (p.0.try_into().unwrap(), p.1.try_into().unwrap())
}
