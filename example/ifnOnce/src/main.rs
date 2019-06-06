struct iFn<T> where T: Fn() + 'static {
    function: T,
}
impl<T> iFn<T> where T: Fn() + 'static {
    pub fn new(f:  T) -> iFn< T>  {
        iFn {
            function: f,
        }
    }
    pub fn call(&self) {
        (self.function)();
    }

} 
fn main() {
    let f = iFn::new(|| {
        println!("hello");
    });
    f.call();
}
