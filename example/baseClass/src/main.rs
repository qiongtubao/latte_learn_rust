/*
 int -> i8,u8,i16,u16,i32,u32,i64,u64,i128,u128
 float -> f32,f64
 string -> char string
 array -> vec
 map -> std::collections::HashMap
 struct
 tuple
 Option<T>
*/
#[derive(Debug)]
struct StudentA {
    name: String,
    age: i8
}
#[derive(Debug)]
struct StudentB(String, i8);
use std::collections::HashMap;

fn main() {
    let a: i8 = 128;  //literal out of range for i8
    let b: u8 = 128;  
    let c: i16 = 32767;
    let d: i16 = 65534/2; // warning: literal out of range for i16
    let e: u128 = 340282366920938463463374607431768211455;
    let f: f32 = 1.2345678910111213141516; //1.2345679
    let g: f64 = 1.2345678910111213141516; //1.2345678910111213
    let mut h = "hello".to_string();
    h.push_str("?world");  
    let mut array = vec![1,2,3];
    array[2] = 4;
    let mut student = StudentA{name:"z".to_string(),age: 18};
    let mut map = HashMap::new();
    map.insert(String::from("hello"), String::from("20"));
    let mut student2 = StudentB("x".to_string(), 17);
    student2.1 = 19;
    let mut tuple = (1,2);
    tuple.1 = 3;
    let none :Option<i32> = Option::None;
    let some: Option<i32> = Some(12);
    println!("Hello, world! {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}", a, b ,c ,d, e, f, g, h, array, map, student, student2, tuple, none, some); 
}
 