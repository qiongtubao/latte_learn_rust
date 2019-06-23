#![deny(warnings)]

use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
mod json;
use json::*;

fn main() {
    let mut args = env::args();
    let mut input = String::new();
    if args.len() > 1 {
        let name = args.nth(1).unwrap();
        File::open(&name)
            .and_then(|mut f| f.read_to_string(&mut input))
            .unwrap();
    } else {
        io::stdin().read_to_string(&mut input).unwrap();
    }

    match input.parse() {
        Ok(toml) => {
            let json = convert(toml);
            println!("{}", serde_json::to_string_pretty(&json).unwrap());
        }
        Err(error) => println!("failed to parse TOML: {}", error),
    }
}

