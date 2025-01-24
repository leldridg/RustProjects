#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Associated function
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
    // Methods
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("rect1 is {rect1:?}");

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
}