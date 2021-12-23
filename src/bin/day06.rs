use anyhow::Result;
use std::collections::VecDeque;


fn advance_one_day(mut pop: VecDeque<usize>) -> VecDeque<usize> {
    let numbabies = pop.pop_front().unwrap();
    pop[6] += numbabies;
    pop.push_back(numbabies);
    pop
}


fn main() -> Result<()> {
    let startpop = util::read_one_line("./data/day06.input", ",")?;

    let mut pop: VecDeque<usize> = VecDeque::from(vec![0;9]);
    startpop.into_iter().for_each(|i| pop[i] += 1);

    let p1 = (0..80).fold(
        pop.clone(),
        |acc, _d| advance_one_day(acc));
    println!("Solution part 1: {}", p1.iter().sum::<usize>());

    let p2 = (0..256).fold(
        pop.clone(),
        |acc, _d| advance_one_day(acc));
    println!("Solution part 2: {}", p2.iter().sum::<usize>());

    Ok(())
}
