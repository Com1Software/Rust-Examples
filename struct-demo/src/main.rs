#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 10, height: 20 };
    let rect2 = Rectangle { width: 15, height: 15 };

    println!("rect1: {:?}, area = {}", rect1, rect1.area());
    println!("rect2: {:?}, area = {}", rect2, rect2.area());

    if rect1.is_square() {
        println!("rect1 is a square");
    } else {
        println!("rect1 is not a square");
    }

    if rect2.is_square() {
        println!("rect2 is a square");
    } else {
        println!("rect2 is not a square");
    }
}
