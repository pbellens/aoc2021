use itertools::Itertools;
use std::collections::BTreeMap;
use anyhow::Result;


type Ids = BTreeMap<&'static str, usize>;
type Edges = BTreeMap<usize, Vec<usize>>;

fn main() -> Result<()> {
    let insert_id = |c: &mut Ids, id| {
        match c.get(id) {
            Some(idx) => *idx, 
            None => { 
                let idx = c.len();
                c.insert(id, idx);
                idx
            }
        }
    };

    let insert_edge = |mut c: Edges, a, b| {
            let entry = c.entry(a).or_insert_with(|| Vec::with_capacity(8));
            (b != 0).then(|| entry.push(b));
            c
        };

    let ids = include_str!("../../data/day12.input")
        .lines()
        .fold(
            (
                BTreeMap::from([("start", 0), ("end", 1)]),
                Edges::new()
            ),
            |mut acc, l| { 
                let (b, e) = l.split_once('-').unwrap();
                let (bi, be) = (insert_id(&mut acc.0, b), insert_id(&mut acc.0, e));
                (
                    acc.0,
                    insert_edge(acc.1, bi, be)
                )
            });

    println!("{:?}", ids);

    Ok(())
}

