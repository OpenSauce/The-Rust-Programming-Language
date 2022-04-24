use std::fmt;

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle { width: 20, ..rect1 };

    println!("Rect1: {}, Area: {}", rect1, rect1.area());
    println!("Can rect1 hold rect2: {}", rect1.can_hold(&rect2));
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Width: {} Height: {}", self.width, self.height)
    }
}
