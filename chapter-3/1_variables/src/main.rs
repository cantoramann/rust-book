fn main() {
    // let x = 5;
    let mut x = 5;
    println!("Value of x is: {x}");
    x = 6;
    println!("The new value of x is: {x}");

    /*
    constants: declare with const keyword

    constants may be set only to a constant expression,
    not the result of a value that could only be computed at runtime...
    */
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    /*
    shadowing: overwrite variables over scope

    the change in the inner scope is only effective within this scope.
    outside of the scope, things get back to how they were.
     */
    let y = 5;

    let y = y + 1;
    // let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }
    println!("The value of y is: {y}");

    /*
    types: conversion within variables and mutable variables
     */
    let spaces = "   ";
    let spaces = spaces.len();

    /*
    let mut spaces2 = "   ";
    spaces2 = spaces.len();

    this won't work due to type mismatch
     */
}
