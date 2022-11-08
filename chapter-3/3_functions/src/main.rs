fn main() {
    another_function(5);
    print_labeled_measurement(5, 'h');

    // x = y = 6 - this does not work in Rust
    let x = five();
    println!("The value of five() is: {x}");

    let y = expression();
    println!("The value of expression() is: {y}");

    let z = plus_one(10);
    println!("The value of plus_one() function is: {z}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    // -> return value is a must
    5 // return values do not end with semi
}

fn expression() -> i32 {
    // -> return value is a must

    let y = {
        let x = 3;
        x + 1
    };
    y // return values do not end with semi
}

fn plus_one(x: i32) -> i32 {
    x + 1 // return values do not end with semi
}
