#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    /*
     * println! macro takes a reference
     * dbg! macro takes ownership and later returns it
     */
    println!("rect1 is {:#?}", rect1);
    dbg!(&rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale), // you can also have custom modifications to debug prints
        height: 50,
    };
    dbg!(&rect2);
}

/*
* keep in mind that we can also pass by value (by changing both the function signature and call)
*/
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
