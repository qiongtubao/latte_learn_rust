use std::cmp;
//
#[derive(Copy, Eq)]
enum Level {
    Error = 1, 
    Warn,
    Info,
    Debug,
    Trace,
}
//实现Copy
impl Clone for Level {
    #[inline]
    fn clone(&self) -> Level {
        *self
    }
}
impl PartialEq for Level {
    // *self 需要实现Copy
    #[inline]
    fn eq(&self, other: &Level) -> bool {
        *self as usize == *other as usize
    }
}

impl Ord for Level {
    #[inline]
    fn cmp(&self, other: &Level) -> cmp::Ordering {
        (*self as usize).cmp(&(*other as usize))
    }
}

impl PartialOrd for Level {
    #[inline]
    fn partial_cmp(&self, other: &Level) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
    #[inline]
    fn lt(&self, other: &Level) -> bool {
        (*self as usize) < *other as usize
    }

    #[inline]
    fn le(&self, other: &Level) -> bool {
        *self as usize <= *other as usize
    }

    #[inline]
    fn gt(&self, other: &Level) -> bool {
        *self as usize > *other as usize
    }

    #[inline]
    fn ge(&self, other: &Level) -> bool {
        *self as usize >= *other as usize
    }
}

fn main() {
    println!("eq! {:?}", Level::Error == Level::Warn);
    println!("than! {:?}", Level::Error < Level::Warn);
    println!("inline! {:?}", Level::Error.partial_cmp(&Level::Warn));
}
