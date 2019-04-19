use std::fmt::{Formatter, Display, Result};
//自己实现fmt 
struct City {
    name: &'static str,
    lat: f32,
    lon: f32
}
impl Display for City {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let lat_c = if self.lat >= 0.0 { 'N'  } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };
        write!(f, "{}: {:.3}°{} {:.3}°{}",
               self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}
//给struct 追加fmt
#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}
fn main() {
    println!("{}", 1);
    println!("{:o}",9);
    println!("{:x}", 255);
    println!("{:X}", 255);
    println!("{:p}", &0);
    println!("{:b}", 15);
    println!("{:e}", 10000f32);
    println!("{:E}", 10000f32);
    println!("{:?}", "test");
    println!("{:#?}", ("test1", "test2"));
    println!("{a} {b} {b}", a = "x", b = "y");

    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", *city);
    }


    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display
        println!("{:?}", *color);
    }
}
