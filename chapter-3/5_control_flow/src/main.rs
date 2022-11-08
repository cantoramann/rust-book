fn main() {
    println!("IF CONDITIONS");
    regularif();
    ifelse();
    conditional_equality();

    wrong(); // to avoid build warning

    println!("LOOPS");
    loop_basic();
    break_outer_from_inner();
    while_loop();
    array_in_loop();
}

fn regularif() {
    println!("Reguler if - else");

    let number = 7;

    println!("number is: {number}");
    if number < 5 {
        println!("condition is less than 5");
    } else {
        println!("condition is greater or equal to 5");
    }
    println!("\n");
}

fn ifelse() {
    println!("If - else if - else");

    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    println!("\n");
}

fn wrong() {

    // if number {
    //     println!("number was three");
    // } -- this does not compile because number is an int
}

fn conditional_equality() {
    println!("Conditional equality");
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
    println!("\n");
}

fn loop_basic() {
    println!("Basic loop");
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // result = counter * 2 when breaks
        }
    };
    println!("The result is {result}");
    println!("\n");
}

fn break_outer_from_inner() {
    println!("Break outer from inner");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break; // break the inner loop
            }
            if count == 2 {
                break 'counting_up; // break the outer loop if ever enter here - in this case it will
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
    println!("\n");
}

fn while_loop() {
    println!("While loop");

    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");
    println!("\n");
}

fn array_in_loop() {
    println!("Iterate array");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    println!("A better way, though:");
    for element in a {
        println!("the value is: {element}");
    }
    println!("\n");
}
