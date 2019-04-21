extern crate threadpool;
use threadpool::ThreadPool;
use std::sync::mpsc::channel;
fn main() {
    let n_workers = 4;
    let n_jobs = 8;
    let pool = ThreadPool::new(n_workers);
    let (tx, rx) = channel();
    for _ in 0..n_jobs {
        let tx = tx.clone();
        pool.execute(move || {
            tx.send(1).expect("channel will be there waiting for the pool");
        });
    }
    assert_eq!(rx.iter().take(n_jobs).fold(0, |a,b| a + b), 8);
    println!("Hello, world!");
}
