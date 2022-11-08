fn main() {
    /*
    if we do not include the u32 below, Rust cannot know what data type to cast.
    */
    let _: u32 = "42".parse().expect("Not a number!");

    /*
    scalar types: int, float, bools, chars
    */

    // let x = 2.0; // f64
    // let y: f32 = 3.0; // f32
    // let sum = 5 + 10;

    // subtraction
    println!("Floats!");
    let difference = 95.5 - 4.3;
    println!("95.5 - 4.3: {difference}");

    // multiplication
    let product = 4 * 30;
    println!("4 * 30: {product}");

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0
    let nonfloored = 2.0 / 3.0;
    println!("56.7 / 32.2: {quotient}");
    println!("2 / 3: {floored}");
    println!("2.0 / 3.0: {nonfloored}");

    // remainder
    let remainder = 43 % 5;
    println!("43 % 5: {remainder}");
    println!("\n");

    // booleans
    println!("Booleans!");
    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("true without explicit cast: {t}");
    println!("false with explicit cast: {f}");
    println!("so explicit casting does not matter in this case");
    println!("\n");

    // chars
    println!("Chars!");
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("character without explicit cast: {c}");
    println!("character with explicit cast: {z}");
    println!("emoji character without explicit cast: {heart_eyed_cat}");
    println!("\n");

    // tuples
    println!("Tuples!");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (t_first, _, _) = tup;
    println!("The first value of tup1 is: {t_first}");

    let tup2: (i32, f64, u8) = (500, 6.4, 1);
    let six_point_four = tup2.1;
    println!("The second value of tup2 is: {six_point_four}");
    // println!("The second value of tup2 is: {tup2.1}"); - does not compile
    println!("\n");

    // arrays
    let arr1 = [1, 2, 3, 4, 5];
    // let arr2 = [
    //     "January",
    //     "February",
    //     "March",
    //     "April",
    //     "May",
    //     "June",
    //     "July",
    //     "August",
    //     "September",
    //     "October",
    //     "November",
    //     "December",
    // ];
    // let arr3 = [3; 5]; // equal to [3, 3, 3, 3, 3]
    // let arr4: [i32; 5] = [1, 2, 3, 4, 5]; // declare type of array items

    let arr1item0 = arr1[0];
    let arr1item1 = arr1[1];
    println!("index 0 of arr0: {arr1item0}");
    println!("index 1 of arr0: {arr1item1}");
    // ! accessing an index out of bounds panicks and exits.
}
