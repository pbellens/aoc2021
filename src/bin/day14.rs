
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
    let bla = (0..40)
        .fold(
            templ,
            |acc, _| acc.iter()
                .fold(
                    acc.clone(),
                    |mut m, (&k, &v)| {
                        let n = rules[&k];
                        let l = m.entry((k.0, n)).or_insert(0);
                        *l += 1;
                        let r = m.entry((k.1, n)).or_insert(0);
                        *r += 1;
                        m }));

    println!("{:?}", bla);

 
    return Ok(())
}
