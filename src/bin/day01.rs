use anyhow::Result;
use itertools::Itertools;

fn sliding_window(size: usize) -> Result<usize> {
    Ok(util::read_one_per_line::<u32>("data/day01.input")?
        .windows(size)
        .filter(|w| w[0] < w[size-1])
        .collect_vec()
        .len())

}

fn main() -> Result<()> {
    println!("Solution for part 1: {}", sliding_window(2)?);
    println!("Solution for part 2: {}", sliding_window(4)?);

    Ok(())
}
