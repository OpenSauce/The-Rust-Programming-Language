use std::fmt;

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Rect1: {}, Area: {}", rect1, area(&rect1))
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Width: {} Height: {}", self.width, self.height)
    }
}
