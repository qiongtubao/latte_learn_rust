use std::fs;
use toml::Value as TomlValue;
use std::collections::HashMap;
use std::path;
use std::path::PathBuf;
use fs_extra::dir;

const LIB_SRC_PATH : &'static str= "/Users/zhouguodong/.cargo/registry/src/github.com-1ecc6299db9ec823";
type Version = (i32, i32, i32);
fn str_to_version(version: &String) -> Version {
    if version == "*" {
        return (-1, -1, -1);
    }
    let split: Vec<&str> = version.split(".").collect();
    return (split.get(0).unwrap_or(&"-1").parse().unwrap_or(-1),
            split.get(1).unwrap_or(&"-1").parse().unwrap_or(-1), -1);
}
fn find_path(name: &String, version: &String) -> Option<String> {
    let path = path::Path::new(LIB_SRC_PATH);
    let regex = regex::Regex::new(&format!(r#"{}-(?P<f>\d+).(?P<s>\d+).(?P<t>\d+)"#,name)).expect("regex");
    let version = str_to_version(version);
    let mut result_file = None;
    let mut result_version = (-1, -1, -1);
    println!("find {}  version {:?}", name, &version);
    for entry in path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
//            println!("{:?}", entry.file_name());
            let file_name = entry.file_name().to_str().expect("filename to_str").to_string();
            let regex_result = regex.captures(&file_name);
            if regex_result.is_none() {
                continue;
            }
            let cap = regex_result.expect("cap");
            let cap_version: Version = (cap["f"].parse().expect(""), cap["s"].parse().expect(""), cap["t"].parse().expect(""));
            println!("cap version: {:?}", cap_version);
            if version.0 == -1 {
                if cap_version.0 > result_version.0 {
                    result_version = cap_version;
                    result_file = Some(file_name);
                }
                continue;
            }
            if version.0 != cap_version.0 {
                continue;
            }
            if version.1 == -1 {
                if cap_version.1 > result_version.1 {
                    result_version = cap_version;
                    result_file = Some(file_name);
                }
                continue;
            }
            if version.1 != cap_version.1 {
                continue;
            }
            if cap_version.2 > result_version.2 {
                result_version = cap_version;
                result_file = Some(file_name);
            }

        }
    }
    result_file
}
fn readDir(root_path: String, package: String, index: i32) {
    println!("readDir: {}", root_path);
    let cargo_toml = fs::read_to_string(format!("{}/Cargo.toml", root_path.clone())).expect("read cargo toml");
    let mut toml_object: TomlValue = toml::from_str(&cargo_toml).expect("parse toml");
    let dep = toml_object.get_mut("dependencies");
    if dep.is_none() {
        return;
    }
    let dependencies = dep.expect("dependencies");
    let table = dependencies.as_table_mut().expect("table");
    let mut new_dependencies = toml::map::Map::new();
    for (key, value) in table.iter() {
        match value {
            TomlValue::String(x) => {
                let  filename = find_path(key, &x);
                if filename.is_none() {
                    panic!(format!("parse run 'cargo build' ,becase it's not install {}  in [{:?}]", &key, root_path));
                }
                let filename = filename.expect("");
                let pathbuf = PathBuf::from(format!("{}/{}", package.clone(), filename.clone()));
                if !pathbuf.exists() {
                    let options = dir::CopyOptions::new();
                    dir::copy(&format!("{}/{}",LIB_SRC_PATH, &filename), package.clone(), &options).expect("copy error");
                }
                readDir(format!("{}/{}", package.clone(), filename.clone()), package.clone(), 1);
                let mut path_object = toml::map::Map::new();
                if index == 1 {
                    path_object.insert("path".to_string(), TomlValue::String(format!("../{}",filename)));
                }else{
                    path_object.insert("path".to_string(), TomlValue::String(format!("{}/{}",package.clone(),filename)));
                }
                new_dependencies.insert(key.clone(), TomlValue::Table(path_object));
            },
            TomlValue::Table(tab) => {
                println!("version：{:?}", tab);
                if tab.get("version").is_none() || tab.get("path").is_some() {
                    new_dependencies.insert(key.clone(), value.clone());
                    continue
                }
                let version = tab.get("version").expect("version").as_str().expect("version as_str");
                let  filename = find_path(key, &version.to_string());
                if filename.is_none() {
                    panic!(format!("parse run 'cargo build' ,becase it's not install {}  in [{:?}]", &key, root_path));
                }
                let filename = filename.expect("");
                let pathbuf = PathBuf::from(format!("{}/{}", package.clone(), filename.clone()));
                if !pathbuf.exists() {
                    let options = dir::CopyOptions::new();
                    dir::copy(&format!("{}/{}",LIB_SRC_PATH, &filename), package.clone(), &options).expect("copy error");
                }
                readDir(format!("{}/{}", package.clone(), filename.clone()), package.clone(), 1);
                let mut map = tab.clone();
                if index == 1 {
                    map.insert("path".to_string(), TomlValue::String(format!("../{}",filename)));
                }else{
                    map.insert("path".to_string(), TomlValue::String(format!("{}/{}",package.clone(),filename)));
                }
                new_dependencies.insert(key.clone(), TomlValue::Table(map));
            },
            _ => {
                println!("未处理 {:?} {:?}",&key, &value);
                new_dependencies.insert(key.clone(), value.clone());
            }
        }
    }
    let toml_map = toml_object.as_table_mut().expect("mut ");
    toml_map.remove("dependencies");
    toml_map.insert("dependencies".to_string(), TomlValue::Table(new_dependencies));
    fs::write(format!("{}/Cargo.toml" ,root_path), toml::to_string(&toml_object).expect("to_string"));
}
fn main() {
    fs::create_dir("./rust_modules");
    readDir("./".to_string(), "./rust_modules".to_string(), 0);
}
