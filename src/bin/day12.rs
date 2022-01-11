use itertools::Itertools;
use anyhow::Result;

fn neighbors(x: usize, y: usize, m: usize, n: usize) -> Vec<(usize, usize)> {
    let sx: i32 = x.try_into().unwrap();
    let sy: i32 = x.try_into().unwrap();
    let sm: i32 = m.try_into().unwrap();
    let sn: i32 = n.try_into().unwrap();

    vec![(-1,-1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1) ]
        .into_iter()
        .map(|(i, j)| (sx + i, sx + j))
        .filter(|&(x,y)| x >= 0 && x < sm && y >= 0 && y < sn)
        .map(|(x, y)| (x.try_into().unwrap(), y.try_into().unwrap()))
        .collect_vec()
}

fn flash(squids:&mut Vec<u32>, x: usize, y: usize, m: usize, n: usize) -> usize {
    squids[x * n + y] = 0;
    neighbors(x, y, m, n)
        .into_iter()
        .fold(
            1,
            |acc, (nx, ny)| 
                match squids.get_mut(nx * n + ny) {
                    Some(s) if *s > 0 => {
                        *s += 1;
                        acc + (*s > 9).then(|| flash(squids, nx, ny, m, n)).unwrap_or(0)
                    },
                    _ => acc,
                })
}

const NEXT: [(isize, isize); 8] = [(0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1)];
fn flash2(map: &mut Vec<u32>, x: usize, y: usize, m: usize, n: usize) -> usize {
    println!("flahs2 {} {} dim {} {}", x, y, m, n);
    map[x * n + y] = 0;
    NEXT.iter()
        .map(|(xx, yy)| ((x as isize + xx) as usize, (y as isize + yy) as usize))
        .fold(1, |acc, (x, y)| {
            println!("checking {} {} {}", x, y, x * n + y);
            match map.get_mut(x * n + y) {
                Some(cell) if *cell > 0 => {
                    *cell += 1;
                    acc + (*cell > 9).then(|| flash(map, x, y, m, n)).unwrap_or(0)
                }
                _ => { println!("\t aha"); acc }
            }
        })
}

fn main() -> Result<()> {
    const M: usize = 10;
    const N: usize = 10;
    
    let mut squids = include_str!("../../data/day11.input")
        .lines()
        .flat_map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();

    println!("squids: {:?}", squids);

    let res = (0..100).fold(
        0,
        |acc, _| { 
            squids.iter_mut().for_each(|s| { *s += 1; } ); 
            acc + (0..M)
                .flat_map(move |x| (0..N).map(move |y| (x,y)))
                .fold(
                    0,
                    |acc, (x, y)| { 
                        acc + (squids[x * N + y] > 9).then(|| flash2(&mut squids, x, y, M, N)).unwrap_or(0)
                    })
        });
        
    println!("{}", res);

    Ok(())
}
