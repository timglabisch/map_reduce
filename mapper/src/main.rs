extern crate regex;
use std::io;
use std::io::prelude::*;
use regex::Regex;

fn main() {

    let stdin = io::stdin();
    let regex = Regex::new(r"(\d+)$").expect("failed to compile regex");

    for line in stdin.lock().lines() {

        let line : String = line.expect("failed to read line");
        let captures = regex
                        .captures(&line)
                        .expect("failed to capture index")
                        .at(0)
                        .expect("failed to read int");

        println!("{}\t1", captures);
    }
}
