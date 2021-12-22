use std::env;
use std::fs::File;
use std::error::Error;
use std::io::BufReader;
use std::io::prelude::*;

fn count_words<R: Read>(reader: R) 
    -> Result<i32, Box<dyn Error>> {
    let bufreader = BufReader::new(reader);
    let mut wc: i32 = 0;
    for l in bufreader.lines() {
        for _w in l?.split_whitespace() {
            wc += 1;
        }
    }

    Ok(wc)
}

fn main() -> Result<(), Box<dyn Error>> {
    for fname in env::args().skip(1) {
        let reader = File::open(&fname)?;
        let wc = count_words(reader)?;
        println!("Found {} words", wc);
    }
    Ok(())
}
