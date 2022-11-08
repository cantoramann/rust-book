fn main() {
    /*
    string literals

    - we know the size (+)
    - not dynamic, faster (+)
    - we may not always know the value in the compile time, string literals do not let us handle this (-)
    */

    string_literal();
    actual_string();
    dynamic_references();
    clone();
    primitive_references();
    ownership_and_function();
    returning_values_and_scope();
}

fn string_literal() {
    println!("String literal");
    let s = "hello"; // s is valid from this point forward
    println!("{}", s);
    println!("\n");
}

fn actual_string() {
    println!("Rust String");

    let mut s = String::from("hello"); // this is stored in the heap. Size is not known.
    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`
    println!("\n");

    /*
    drop() function is called automatically. This is why we do not need to manually delete dynamic variables.
    This is also called Resource Acquisition Is Initialization (RAII)
    */
}

fn dynamic_references() {
    println!("Dynamic references");

    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}, world!", s1); - won't work because s1 is no longer valid
    // ! ownership principle: one owner at a time!
    println!("{}", s2);
    println!("\n");
}

fn clone() {
    println!("Deep copies with clones");

    /*
    If we clone instead, there won't be an issue. These two variables will be different.
    We can freely use them as separate variables.
    */
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
    println!("\n");
}

fn primitive_references() {
    println!("Static references");

    /*
    This works fine, because nothing is in the heap. We know the sizes.
    No ownership principle takes place as in dynamic variables.
    */
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
    println!("\n");
}

fn ownership_and_function() {
    println!("Ownership and functions");
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    // println!("{}", s); - won't compile: s is dropped in the end of takes_ownership function.

    let x = 5; // x comes into scope

    makes_copy(x);
    println!("{}", x);
    println!("\n");
    /*
    This will compile though. Because int is an primitive type, nothing was freed after makes_copy().
    */
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn returning_values_and_scope() {
    println!("Ownership and functions with return values");

    let _ = gives_ownership(); // gives_ownership moves its return
                               // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let _ = takes_and_gives_back(s2); // s2 is moved into
                                      // takes_and_gives_back, which also
                                      // moves its return value into s3
    println!("\n");
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}
