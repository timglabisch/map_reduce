use std::io;
use std::io::prelude::*;

fn main() {

    let stdin = io::stdin();
    let mut last_number : Option <i32> = None;
    let mut last_count = 0;

    for line in stdin.lock().lines() {

        let line : String = line.expect("failed to read line");
        let mut split = line.split("\t");

        let key = split
                    .next()
                    .expect("key must be given")
                    .parse::<i32>()
                    .expect("key must be an int");

        let value = split
                    .next()
                    .expect("value must be given")
                    .parse::<i32>()
                    .expect("value must be an int");

        if last_number.is_none() {
            last_number = Some(key);
        }

        if last_number.unwrap() == key {
            last_count += value;
            continue;
        }

        println!(
            "{}\t{}",
            last_number.expect("number cant be null"),
            last_count
        );

        last_count = 1;
        last_number = Some(key);
    }

    if last_count != 0 && last_number.is_some() {
        println!(
            "{}\t{}",
            last_number.expect("number cant be null"),
            last_count
        );
    }
}
