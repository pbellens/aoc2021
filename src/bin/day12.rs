use itertools::Itertools;
use std::collections::BTreeMap;
use std::collections::HashSet;
use anyhow::Result;


#[derive(Debug, Clone, Copy)]
struct Cave {
    idx: usize,
    small: bool,
}


type Ids = BTreeMap<&'static str, Cave>;
type Edges = BTreeMap<usize, Vec<usize>>;

fn main() -> Result<()> {
    let insert_id = |c: &mut Ids, id| {
        match c.get(id) {
            Some(cave) => cave.idx,
            None => { 
                let idx = c.len();
                c.insert(
                    id, 
                    Cave { 
                        idx: idx, 
                        small: id.chars().all(|c| c.is_lowercase()) });
                idx
            }
        }
    };

    let insert_edge = |mut c: Edges, a, b| {
            let entry = c.entry(a).or_insert_with(|| Vec::with_capacity(8));
            (b != 0).then(|| entry.push(b));
            c
        };

    let (ids, edges) = include_str!("../../data/day12.input")
        .lines()
        .fold(
            (
                BTreeMap::from([("start", Cave { idx: 0, small: true }), ("end", Cave { idx: 1, small: false })]),
                Edges::new()
            ),
            |mut acc, l| { 
                let (b, e) = l.split_once('-').unwrap();
                let (bi, be) = (insert_id(&mut acc.0, b), insert_id(&mut acc.0, e));
                (
                    acc.0,
                    insert_edge(insert_edge(acc.1, bi, be), be, bi)
                )
            });

    let smalls: HashSet<usize> = ids
        .values()
        .filter(|c| c.small)
        .map(|c| c.idx)
        .collect();

    println!("ids: {:?}", ids);
    println!("edges: {:?}", edges);

    {
        let mut paths = vec![];
        let mut cand = vec![vec![0 as usize]];
        while let Some(cs) = cand.pop() {
            let last = cs.last().unwrap();
            
            if last == &1 {
                paths.push(cs.clone());
                continue;
            }
            
            let visited : HashSet<usize> = HashSet::from_iter(cs.clone().into_iter())
                .intersection(&smalls)
                .cloned()
                .collect();
            for e in HashSet::from_iter(edges[&last].clone().into_iter())
                .difference(&visited)
            {
                let mut freshpath = cs.clone();
                freshpath.push(*e);
                cand.push(freshpath);

            }
        }

        println!("Solution part 1: {:?}", paths.len());
    }

    {
        let mut paths = vec![];
        let mut cand = vec![vec![0 as usize]];
        while let Some(cs) = cand.pop() {
            let last = cs.last().unwrap();
            
            if last == &1 {
                paths.push(cs.clone());
                continue;
            }

            let mut m : BTreeMap<usize, usize> = BTreeMap::new();
            for n in cs.iter() {
                if smalls.contains(n) {
                    *m.entry(*n).or_insert(0) += 1;
                }
            }
            let smalled = m.iter().any(|(k, v)| v == &2);

            for e in edges[&last].clone().into_iter()
            {
                let count = m.get(&e).unwrap_or(&0);
                if e != 0 
                    && (!smalls.contains(&e) 
                        || (count < &2 && !smalled)) {
                    let mut freshpath = cs.clone();
                    freshpath.push(e);
                    cand.push(freshpath);
                }

            }
        }

        println!("Solution part 2: {:?}", paths.len());
    }

    Ok(())
}

