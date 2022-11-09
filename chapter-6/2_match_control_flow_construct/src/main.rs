#[derive(Debug)]

enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    println!("Hello, world!");
    // println!("A penny is worth, {} ", value_in_cents(Coin::Penny));
    value_in_cents(Coin::Quarter(UsState::Alaska));
    value_in_cents(Coin::Penny);
    plus_one();
    placeholder_();
}

/*
* if needs to return a boolean.
* match conditions can return anything.
*/
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one() {
    println!("plus_one() starts here");

    let five = Some(5);
    let six = plus_one_helper(five);
    let none = plus_one_helper(None);

    println!("\n");
}

fn plus_one_helper(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn placeholder_() {
    println!("_ placeholder example");

    let dice_roll = 9;
    let res = match dice_roll {
        3 => 6,
        7 => 14,
        _ => 0, // anything other than 3 or 7 will yield 0
    };

    println!("{}", res);
    println!("\n");
}
