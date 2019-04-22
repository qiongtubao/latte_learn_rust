

#![feature(async_await, futures_api)]
use futures::executor::block_on;
async fn print_async() {
     println!("Hello from print_async")
}

fn main() { 
     let future = print_async(); 
    
     block_on(future);
      println!("Hello from main");
}