
extern crate reqwest;
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate url;
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
fn query(query: &str) -> Result<(), reqwest::Error> {
    use url::Url;

let url = Url::parse_with_params("https://crates.io/api/v1/crates",
                                 &[("page", "1"), ("per_page", "10"),("q", query)]).unwrap();
    // let mut uri = "https://crates.io/api/v1/crates?page=1&per_page=10&q=".to_string();
    // uri.push_str(query);
    let body: Content = reqwest::get(&url.into_string())?
    .json()?;

    println!("body = {:?}", body);
    Ok(())
}
fn main()  ->  Result<(), reqwest::Error> {
    
    query("tokio")
    
}

