use anyhow::Result;
use pathfinding::prelude::dijkstra;


fn successors(pos: (i32, i32), nrows: i32, ncols: i32) -> Vec<(i32, i32)> {
    vec![(0, 1), (1, 0), (-1, 0), (0,-1)]
        .iter()
        .map(|(x,y)| (pos.0 + x, pos.1 + y))
        .filter(|(x, y)| *x >= 0 && *y >= 0 && *x < nrows * 5 && *y < ncols * 5)
        .map(|(x, y)| (x, y))
        .collect()
}

fn main() -> Result<()> {
    let cave1 : Vec<Vec<u32>> = include_str!("../../data/day15.input")
        .trim()
        .lines()
        .map(|l| l.bytes().map(|b| u32::from(b - b'0')).collect())
        .collect();
    let nrows: i32 = cave1.len().try_into().unwrap();
    let ncols: i32 = cave1.first().unwrap().len().try_into().unwrap();


    let cost = dijkstra(
        &(0,0),
        |&(x, y)| successors((x, y), nrows, ncols)
            .iter()
            .map(|&(x,y)| {
                let ix: usize = (x % nrows).try_into().unwrap();
                let iy: usize = (y % ncols).try_into().unwrap();
                ((x, y), ((cave1[ix][iy] as i32 + (x / nrows) + (y / ncols) - 1) % 9 + 1) as u32)})
            .collect::<Vec<_>>(),
        |&(x, y)| (x, y) == (nrows * 5 - 1, ncols * 5 - 1));

    println!("{:?}", cost);

    Ok(())
}
