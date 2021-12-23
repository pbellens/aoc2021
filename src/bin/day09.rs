use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::collections::HashSet;


fn neighbors(coord: (i32, i32), nrows: i32, ncols: i32) -> Vec<(usize, usize)> {
    vec![(0,1), (1,0), (-1, 0), (0, -1)]
        .iter()
        .map(|(x,y)| (coord.0 + x, coord.1 + y))
        .filter(|(x, y)| *x >= 0 && *y >= 0 && *x < nrows && *y < ncols)
        .map(|(x, y)| (x.try_into().unwrap(), y.try_into().unwrap()))
        .collect_vec()
}



fn main() -> Result<()> {
    let s = include_str!("../../data/day09.input")
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();

    let dimx = i32::try_from(s.len()).unwrap();
    let dimy = i32::try_from(s.first().unwrap().len()).unwrap();


    let mut pits = vec![];
    for (r, l) in s.iter().enumerate() {
        let row = i32::try_from(r).unwrap();
        for (c, elem) in l.iter().enumerate() {
            let col = i32::try_from(c).unwrap();
            if neighbors((row, col), dimx, dimy)
                .into_iter()
                .all(|(x, y)| s[x][y] > *elem) {
                    pits.push((r, c));
                }
        }
    }

    println!("Solution part 1: {}", pits.iter().fold(0, |acc, &(x, y)| acc + s[x][y] + 1));


    let mut basins: Vec<usize> = Vec::new();
    let mut seen: HashSet<(usize, usize)> = HashSet::new();

    for pit in pits.iter() {
        seen.insert(*pit);

        let mut visit = neighbors(util::make_idx(*pit), dimx, dimy)
            .into_iter()
            .map(|(a, b)| ((a, b), s[pit.0][pit.1]))
            .collect_vec();

        // Count the basin base, so start at 1
        let mut bass = 1;
        while let Some((cand, height)) = visit.pop() {
            if !seen.contains(&cand) {
                let val = s[cand.0][cand.1];
                if val > height && val != 9 {
                    bass += 1;

                    visit.extend(
                        neighbors(util::make_idx(cand), dimx, dimy)
                            .into_iter()
                            .map(|(a, b)| ((a, b), val)),
                    );

                    seen.insert(cand);
                }
            }
        }

        basins.push(bass);
    }

    println!("Solution part 2: {:?}", basins.iter().sorted().rev().take(3).product::<usize>());

    Ok(())
}
