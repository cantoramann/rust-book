// What's an IP address? https://en.wikipedia.org/wiki/IP_address
// for more reference about Rust enums: https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html

enum IpAddr {
    V4(String),
    V6(String),
}

enum IpAddrBetter {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// * It is also possible to implement an enum!
impl Message {
    fn call(&self) {
        println!("inside the call() in Message");
    }
}

fn main() {
    println!("IP examples start here");
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    let homeBetter = IpAddrBetter::V4(127, 0, 0, 1);
    let loopbackBetter = IpAddrBetter::V6(String::from("::1"));

    println!("Message examples start here");

    let m = Message::Write(String::from("hello"));
    /*
     * m holds the type (write) and the data ("hello")
     * This is simpler than creating a struct and holding the type and the data.
     * No need to create a struct to store both. This is kinda cool.
     */

    m.call();
}

// abstractly speaking, this is a replacement to null.
enum Option<T> {
    None,
    Some(T),
}

fn some() {
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    // let sum = x + y; - won't compile because i8 and Option<i8> are different types.
}
