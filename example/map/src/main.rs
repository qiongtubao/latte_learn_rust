use std::collections::HashMap;
fn map() {
    let mut map:HashMap<&str, i32> = HashMap::new();
    assert_eq!(map.len(), 0);
    //直接添加
    map.insert("王一", 1);
    map.insert("王一", 2);
    assert_eq!(map["王一"], 2);
    assert_eq!(map.len(), 1);
    //获得Entry 对象  插入
    map.entry("张二").or_insert(12);
    assert_eq!(map.len(), 2);
    //删除
    map.remove("王一");
    assert_eq!(map.len(), 1);
    //通过key获取value
    assert_eq!(map["张二"], 12);
    //map["张二"] = 13; Error
    if let Some(x) = map.get_mut("张二") {
        *x = 13;
    }
    assert_eq!(map["张二"], 13);
    //遍历修改数据
    for value in map.values_mut() {
        *value = *value + 10;
    }
    //遍历
    for key in map.keys() {
        println!("{:?} {:?}",key, map[key]);
    }
    //hashmap -> Vec<(_, _)>
    let vec: Vec<(_, _)> = map.into_iter().collect();
    // println!("vec :{:?}", vec);
    assert_eq!(vec, [("张二", 23)]);    
    // Vec<(_, _)> -> hashmap
    //collect 必须自己定义好类型 不能通过自动推断了
    let map1:HashMap<&str, i32> = vec.iter().cloned().collect();  
    println!("vec :{:?}", map1);
    //Vec<Key> Vec<Value> -> hashmap
    let keys = vec!["WaySLOG", "Mike", "Elton"];
    let values = vec![60, 80, 100];
    let score_map: HashMap<_, _> = keys.iter()
    .zip(values.iter())
    .collect();
    println!("map {:?}", score_map);
    //range -> hashmap
    let mut mp: HashMap<i64, i64> = (0..8).map(|x|(x, x*10)).collect();
    //过滤
    mp.retain(|&k, _| k % 2 == 0);
    println!("range {:?}", mp);
    mp.reserve(10);
    println!("reserve {:?}",mp.capacity());
}
fn main() {
    map();
}
