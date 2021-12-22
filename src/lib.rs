use anyhow::Result;
use std::str::FromStr;

pub fn read_one_per_line<T: FromStr>(path: &str) -> Result<Vec<T>> {
    Ok(std::fs::read_to_string(path)?
        .lines()
        .filter_map(|line| line.parse::<T>().ok())
        .collect())
}

