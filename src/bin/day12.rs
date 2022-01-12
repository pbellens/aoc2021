use itertools::Itertools;
use std::collections::BTreeMap;
use anyhow::Result;

type Ids = BTreeMap<&'static str, usize>;

fn main() -> Result<()> {
    let insert_id = |mut c: Ids, id| {
        match c.get(id) {
            Some(_) => c,
            None => { 
                c.insert(id, c.len() + 2); 
                c 
            }
        }
    };

    let insert_ids = |mut c: Ids, id1, id2| insert_id(insert_id(c, id1), id2);

    let ids = include_str!("../../data/day12.input")
        .lines()
        .fold(
            BTreeMap::from([("start", 0), ("end", 1)]),
            |acc, l| { 
                l.split_once('-').map(|(b, e)| { insert_ids(acc, b, e) }).unwrap()
            });

    println!("{:?}", ids);

    Ok(())
}

