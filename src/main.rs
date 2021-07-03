// Rectangle as struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Method to return rectangle area
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    // New rectangle
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // Print (debug) info on rectangle
    println!("rect1 is {:?}", rect1);

    // Print area
    println!(
        "The area of the rect1 is {} square pixels.",
        rect1.area()
    );
    println!(
        "The area of the rect2 is {} square pixels.",
        rect2.area()
    );
    println!(
        "The area of the rect3 is {} square pixels.",
        rect3.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}