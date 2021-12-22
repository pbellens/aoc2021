use anyhow::Result;
use std::str::FromStr;
use std::ops::Range;

#[derive(Debug, Clone)]
struct ReportLine(Vec<u32>);


impl FromStr for ReportLine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self(s
            .chars()
            .fold(
                vec![],
                |mut acc, c| { acc.push(c.to_digit(10).unwrap()); acc })))

    }
}

impl From<&ReportLine> for u32 {
    fn from(l: &ReportLine) -> u32 {
        l.0.iter()
            .fold(
                0,
                |acc, b| acc * 2 + b)
    }
}

impl AsRef<Vec<u32>> for ReportLine {
    fn as_ref(&self) -> &Vec<u32> {
        &self.0
    }
}

#[derive(Debug, Clone)]
struct ReportSummary(Vec<u32>);

impl From<&ReportSummary> for u32 {
    fn from(l: &ReportSummary) -> u32 {
        l.0.iter()
            .fold(
                0,
                |acc, b| acc * 2 + b)
    }
}

impl From<Vec<u32>> for ReportSummary {
    fn from(l: Vec<u32>) -> ReportSummary {
        ReportSummary(l)
    }
}

impl ReportSummary {
    fn summarize_ranged(r: &Vec<ReportLine>, range: Range<usize>, linesize: usize) -> (usize, ReportSummary) {
        let tmp = r[range]
            .iter()
            .fold(
                (0, vec![0; linesize]),
                |acc, l| (acc.0 + 1, acc.1.iter().zip(l.as_ref().iter()).map(|(&a, &b)| a + b).collect()));

        (tmp.0, ReportSummary::from(tmp.1))
    }

    fn summarize(r: &Vec<ReportLine>, linesize: usize) -> (usize, ReportSummary) {
        ReportSummary::summarize_ranged(r, 0..r.len(), linesize)
    }
}

fn iterative_filter<F>(mut ls: Vec<ReportLine>, vsize: usize, f: F) -> Vec<ReportLine>
    where F: Fn(&ReportLine, usize, u32) -> bool 
{
    for idx in 0..vsize {
        let (size, s) = ReportSummary::summarize(&ls, vsize);
        let thres : usize = s.0[idx].try_into().unwrap();
        let dist = if thres >= (size + 1) / 2 { 1 } else { 0 };

        ls = ls
            .into_iter()
            .filter(|rl| f(rl, idx, dist))
            .collect();

        if ls.len() == 1 {
            break
        }
    }

    ls
}


fn main() -> Result<()> {
    let vsize: usize = 12;

    let report = util::read_one_per_line::<ReportLine>("data/day03.input")?; 

    // part 1
    let (size, summary) = ReportSummary::summarize(&report, vsize);
    let half : u32 = (size / 2).try_into()?;

    let gamma = ReportSummary::from(summary.0.iter().map(|s| if *s >= half { 1 } else { 0 }).collect::<Vec<u32>>()); 
    let epsilon = ReportSummary::from(summary.0.iter().map(|s| if *s < half { 1 } else { 0 }).collect::<Vec<u32>>()); 
    println!("Solution for part 1: {}", u32::from(&gamma) * u32::from(&epsilon));

    // part2
    let q = u32::from(&iterative_filter(report.clone(), vsize, |rl, idx, v| rl.0[idx] == v)[0]);
    let p = u32::from(&iterative_filter(report.clone(), vsize, |rl, idx, v| rl.0[idx] != v)[0]);
    println!("Solution for part 2: {}", q * p);

    return Ok(())
}
