#![deny(warnings)]
use std::fs::File;
use std::io::prelude::*;
use toml::Value as Toml;
use serde_derive::Deserialize;
use std::collections::HashMap;
#[derive(Debug,Deserialize)]
struct Cargo {
    dependencies: HashMap<String, String>,
}
fn main() {
    let mut input = String::new();
    let name = "Cargo.toml";
     File::open(&name)
            .and_then(|mut f| f.read_to_string(&mut input))
            .unwrap();

    match toml::from_str(&input) {
        Ok(t) => {
            parse(t);
        }
        Err(error) => println!("failed to parse TOML: {}", error),
    }
}
#[derive(Debug,Deserialize)]
struct Info {
    path: Option<String>,
    version: Option<String>,
    features: Option<Vec<String>>,
}
#[derive(Debug, Deserialize)]
enum Version {
    String(String),
    Info(Info),
}

impl From<String> for Version {
    fn from(version: String) -> Version {
        Version::String(version)
    }
}
// impl From<Toml::String> for Version {
//     fn from(version: Toml::String) -> Version {
//         Version::String(version.to_string())
//     }
// }
impl From<Info> for Version {
    fn from(version: Info) -> Version {
        Version::Info(version)
    }
}
impl<'a> From<&'a str> for Version {
    fn from(str: &'a str) -> Version {
        Version::String(str.to_string())
    }
}
impl From<Toml> for Version {
    fn from(_: Toml) -> Version {
        let info = Info {
            path: None,
            version: None,
            features: None
        };
        Version::Info(info)
    }
}
fn parse(t: Cargo) {
    println!("{:?}", t);
    //let mut hash = HashMap::new();
    // match t.dependencies {
    //     Toml::Table(table) => {
    //         for (i, n) in table.into_iter().enumerate() {
    //             hash.insert(i , n);
    //         }
    //         // table.into_iter().for_each(|(k, v)| hash.insert(k, v););
    //     },
    //     _ => {
    //         println!("{:?}", "文件格式错误");
    //     }
    // }
    println!("hash: {:?}", t.dependencies);
}

