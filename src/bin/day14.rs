#![feature(int_roundings)]
use anyhow::Result;
use std::collections::HashMap;


fn main() -> Result<()> {
    let (templ, rulestr) = include_str!("../../data/day14.input")
        .trim()
        .split_once("\n\n")
        .unwrap();
    let rules: HashMap<(u8, u8), u8> = rulestr
        .lines()
        .map(|l| {
            let (left, right) = l.split_once(" -> ").unwrap();
            let (left, right) = (left.as_bytes(), right.as_bytes());
            ((left[0], left[1]), right[0]) })
        .collect();

    //println!("orig templ {:?}", templ);

    let templ = templ
        .as_bytes()
        .windows(2)
        .map(|w| (w[0], w[1]))
        .fold(
            HashMap::new(),
            |mut m, p| {
                let e = m.entry(p).or_insert(0);
                *e += 1;
                m});

    let (min, max) = (0..40)
        .fold(
            templ,
            |acc, _| acc.iter()
                .fold(
                    HashMap::new(),
                    |mut m, (&k, &v)| {
                        let n = rules[&k];
                        let l = m.entry((k.0, n)).or_insert(0_usize);
                        *l += v;
                        let r = m.entry((n, k.1)).or_insert(0_usize);
                        *r += v;
                        m }))
        .iter()
        .fold(
            HashMap::new(),
            |mut acc, (&(l, r), c)| {
                let lx = acc.entry(l).or_insert(0_usize);
                *lx += c;
                let rx = acc.entry(r).or_insert(0_usize);
                *rx += c;
                acc})
        .iter()
        .map(|(_, &c)| usize::unstable_div_ceil(c, 2))
        .fold(
            (usize::MAX, usize::MIN),
            |acc, c| (if acc.0 < c { acc.0 } else { c }, if acc.1 > c { acc.1 } else { c }));

    println!("Solution day 14 part 2: {:?}", max - min);
 
    return Ok(())
}
