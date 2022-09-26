#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
fn main() {
    println!("Hello, world!");
    let w = 30;
    let l = 50;
    let rect = (30, 50);
    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };
    let s = Rectangle::square(3);
    println!("{}", area_common(w, l));
    println!("{}", area_tuple(rect));
    println!("{:#?}", area_struct(&rectangle));
}

// common
fn area_common(width: u32, length: u32) -> u32 {
    width * length
}
// tuple
fn area_tuple(dim: (u32, u32)) -> u32 {
    dim.0 * dim.1
}
// struct
fn area_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}