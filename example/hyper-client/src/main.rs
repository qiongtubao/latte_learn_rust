#![deny(warnings)]
extern crate hyper;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use hyper::{Client,Request};
use hyper::rt::{self, Future, Stream};
use hyper::header::USER_AGENT;
extern crate hyper_tls;
fn main() {
    // let url = "http://jsonplaceholder.typicode.com/users".parse().unwrap();
    let url = "https://crates.io/api/v1/crates?page=1&per_page=10&q=tokio".parse().unwrap();
    
    let fut = fetch_json(url)
        .map(|users| {
            // print users
            println!("users: {:#?}", users);

            // print the sum of ids
            // let sum = users.iter().fold(0, |acc, user| acc + user.id);
            // println!("sum of ids: {}", sum);
        })
        // if there was an error print it
        .map_err(|e| {
            match e {
                FetchError::Http(e) => eprintln!("http error: {}", e),
                FetchError::Json(e) => eprintln!("json parsing error: {}", e),
            }
        });
        // use the parsed vector
        
    // Run the runtime with the future trying to fetch, parse and print json.
    //
    // Note that in more complicated use cases, the runtime should probably
    // run on its own, and futures should just be spawned into it.
    rt::run(fut);
}

fn fetch_json(url: hyper::Uri) -> impl Future<Item=Content, Error=FetchError> {
    let https = hyper_tls::HttpsConnector::new(4).unwrap();
    let client = Client::builder()
            .build::<_, hyper::Body>(https);
    let mut req = Request::new(hyper::Body::empty());
    *req.uri_mut() = url;
    req.headers_mut().insert(USER_AGENT, "latte".parse().unwrap());
    client
        // Fetch the url...
        .request(req)
        // And then, if we get a response back...
        .and_then(|res| {
            // asynchronously concatenate chunks of the body
            res.into_body().concat2()
        })
        .from_err::<FetchError>()
        // use the body after concatenation
        .and_then(|body| {
            // try to parse as json with serde_json
            println!("body: {:?}", body);
            let users = serde_json::from_slice(&body)?;
           
            Ok(users)
        })
        .from_err()
}

#[derive(Serialize, Deserialize, Debug)]
struct Crate {
    id: String,
    repository: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Content {
    crates: Vec<Crate>,
}


// Define a type so we can return multiple types of errors
enum FetchError {
    Http(hyper::Error),
    Json(serde_json::Error),
}

impl From<hyper::Error> for FetchError {
    fn from(err: hyper::Error) -> FetchError {
        FetchError::Http(err)
    }
}

impl From<serde_json::Error> for FetchError {
    fn from(err: serde_json::Error) -> FetchError {
        FetchError::Json(err)
    }
}
