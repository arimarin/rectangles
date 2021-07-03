// Rectangle as struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // new rectangle
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // Print (debug) info on rectangle
    println!("rect1 is {:?}", rect1);

    // Print area
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

// Function to return rectlange area
fn area(rectlange: &Rectangle) -> u32 {
    rectlange.width * rectlange.height
}