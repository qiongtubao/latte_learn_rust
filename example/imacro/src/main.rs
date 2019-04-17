//定义宏 hashmap
macro_rules! hashmap {
    ($( $key: expr => $val: expr), *) => {
        {
            let mut map = ::std::collections::HashMap::new();
            $(map.insert($key, $val);)*
            map
        }
    }
}
fn main() {
    let hash = hashmap!["hello" => 1, "world" => 2];
    println!("{:?}", hash);
}
