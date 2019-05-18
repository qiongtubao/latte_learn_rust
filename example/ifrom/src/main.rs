#[derive(Debug, PartialEq)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}
impl From<i8> for Number {
    fn from(item: i8) -> Self {
        Number {value: i32::from(item) }
    }
}

fn main() {
    let num = Number::from(30);
    assert_eq!(num, Number{value: 30});
    let a: i8 = 10;
    let num = Number::from(a);
    assert_eq!(num, Number{value: 10});
    let num: Number = a.into();
    assert_eq!(num, Number{value: 10});
}
