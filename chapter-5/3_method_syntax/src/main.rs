#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

/*
* all functions within the impl block are called "associated functions"
! new keyword is not built into the Rust language - not a keyword here. No need to call
! this name for constructors.
*/
impl Rectangle {
    // * implementation block: functions that belong to the rectangle struct are here.

    // * this is a constructor
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    // * below are member functions
    fn area(&self) -> u32 {
        // self: like Python, gets the self struct as a reference
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
