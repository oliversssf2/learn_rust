#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle{
        Rectangle { width: size, height: size }
    }
    
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    
    let rect2 = Rectangle {
        width: 40*scale,
        height: 60,
    };

    println!(
        "The area of the rectangle is {} square pixel",
        rect1.area()
    );
    println!(
        "The first rectangle can hold the second rectangle: {}",
        rect1.can_hold(&rect2)
    );

    println!(
        "The second rectangle can hold the first rectangle: {}",
        rect2.can_hold(&rect1)
    );


    
    // println!("rect1 is {:#?}", rect1);
}
