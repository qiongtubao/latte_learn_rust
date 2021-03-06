use std::thread::{self};
use std::sync::Arc;
/// 用box 处理?Sized 问题
/// 以及处理iFn<T> 对象handle 
/// 
trait MiddleHandle: Sized + Send{
    // add code here
    fn call(&self);
}

struct Middle {
    handle: Box<Fn() + 'static + Send + Sync>,
}
impl Middle {
    fn new<F>(f: F) -> Middle where
         F: Fn() + 'static + Send + Sync,{
        Middle{handle: Box::new(f)}
    }
}
impl MiddleHandle for Middle {
    fn call(&self) {
        (self.handle)()
    }
}
struct iFn<T, F> where 
    T: Fn() + 'static ,
    F: MiddleHandle + ?Sized + Send + Sync{
    befores: Vec<F>,
    handle: Box<T>,
    afters: Vec<F>,
}


impl<T, F> iFn<T, F> where 
    T: Fn() + 'static ,
    F: MiddleHandle + ?Sized + Send  + Sync{
    pub fn new(f: T) -> iFn<T, F>  {
        iFn {
            befores: vec![],
            handle: Box::new(f),
            afters: vec![],
        }
    }
    pub fn before(&mut self, f: F) {
        self.befores.push(f);
    }
    pub fn after(&mut self, f: F) {
        self.afters.push(f);
    }
    pub fn call(&self) {
        for f in &self.befores {
            f.call();
        }
        (self.handle)();
        for f in &self.afters {
            f.call();
        }
    }
} 
fn main() {
    let mut a = || {
        println!("hello");
    };
    
    let mut f = iFn::new(a);
    let b = Middle::new(|| {
        println!("before");
    });
    f.before(b);
    f.after(Middle::new(|| {
        println!("after");
    }));
    // f.after(|| {
    //     println!("after");
    // });
    let handle = thread::spawn(move || {
        f.call();
    });
    handle.join();
}
