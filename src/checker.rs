use colored::*;
use std::fs::OpenOptions;
use std::io::{prelude::*, BufReader, Read};
pub fn check(token: String) {
    let client = reqwest::blocking::Client::new();
    let response = client.get("https://discord.com/api/v9/users/@me/affinities/guilds").header("authorization", token.clone()).send().unwrap();
    match response.status() {
        reqwest::StatusCode::OK => {
            println!("[{0}] {1} {2}", "+".blue(), "Token is VALID:".green().bold(), token.green().bold());
            write_to_file(&token);
        }
        s => {
            if s == 401 {
                println!("[{0}] {1} {2}", "?".yellow(), "Token is INVALID:".red().bold(), token.blue().bold());
            }
            if s == 403 {
                println!("[{0}] {1} {2}", "?".yellow(), "Token is LOCKED:".red().bold(), token.blue().bold());
            }
        }
    }
}

fn write_to_file(token: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("working.txt")
        .unwrap();

    if let Err(e) = writeln!(file, "{}", &token) {
        println!("Couldn't write to file: {}", e);
    }
}