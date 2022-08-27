use std::fs::File;
use colored::*;
use std::fs::OpenOptions;
use std::io::{prelude::*, BufReader, Read};
mod checker;

fn read_all_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut lines_vec = vec![];
    for line in reader.lines() {
        lines_vec.push(line.unwrap());
    }
    lines_vec
}

fn main() {
        let tokens = read_all_lines("tokens.txt");
        let mut thread_vec = vec![];
        for token in tokens {
            let trd = std::thread::spawn(move || {
                checker::check(token);
            });
            thread_vec.push(trd);
        }
        for trd in thread_vec {
            trd.join();
        }
}