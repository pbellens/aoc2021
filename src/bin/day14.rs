
use anyhow::Result;
use std::collections::HashMap;


fn main() -> Result<()> {
    let (templ, rulestr) = include_str!("../../data/day14.input")
        .trim()
        .split_once("\n\n")
        .unwrap();
    let templ = templ
        .as_bytes()
        .to_vec();
    let rules: Vec<(u8, u8, u8)> = rulestr
        .lines()
        .map(|l| {
            let (left, right) = l.split_once(" -> ").unwrap();
            let (left, right) = (left.as_bytes(), right.as_bytes());
            (left[0], left[1], right[0]) })
        .collect();

    println!("{}", String::from_utf8(templ.clone()).unwrap());
    //println!("{:?}", rules);


    let zaza = (0..10).fold(
        templ,
        |acc, _| std::iter::once(acc[0])
            .chain(acc
                .windows(2)
                .flat_map(|w| {
                    let bla = rules
                        .iter()
                        .find(|&&r| r.0 == w[0] && r.1 == w[1])
                        .unwrap().2;
                    [bla, w[1]]}))
            .collect());
    println!("{}", String::from_utf8(zaza.clone()).unwrap());
    let gaga = zaza.iter().fold(
        HashMap::new(),
        |mut acc, t| {
            let e = acc.entry(t).or_insert(0);
            *e += 1;
            acc
        });

    println!("{:?}", gaga);

    return Ok(())
}
