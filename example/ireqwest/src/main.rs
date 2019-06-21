
extern crate reqwest;
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Crate {
    id: String,
    repository: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Content {
    crates: Vec<Crate>,
}

fn main()  ->  Result<(), reqwest::Error> {
    let body: Content = reqwest::get("https://crates.io/api/v1/crates?page=1&per_page=10&q=tokio")?
    .json()?;

    println!("body = {:?}", body);
    
    Ok(())
}

