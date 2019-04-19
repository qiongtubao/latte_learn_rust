use std::thread;
use std::sync::{Arc, Mutex, mpsc};

fn main() {
    let vec = vec![1_u32,2,3];
    let mut arc = Arc::new(Mutex::new(vec));
    let (tx, rx) = mpsc::channel();
    let data = arc.clone();
    let t = tx.clone();
    let child_thread = thread::spawn(move || {
        println!("Hello, world! pid:{:?}", thread::current().id());
        // panic!("oops");
        let mut d = data.lock().unwrap();
        d[2] += 1; //改变arc值
        t.send("hello");
        "234"
    });
    println!("main pid:{:?}",  thread::current().id());

    let result = child_thread.join();
    let mut c = arc.clone();
    println!("callback lock: {:?}", c);
    // println!("result: {:?}", result.unwrap());
    // assert!(result.is_err());
    match result { 
        Ok(x) => println!("ok {:?}", x),
        Err(x) => println!("err?????????? {:?}", x),
    }
    let rx_result = rx.recv().unwrap();
    println!("rx result:{:?}", rx_result);
}
