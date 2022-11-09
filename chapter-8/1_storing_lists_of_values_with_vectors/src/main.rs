fn main() {
    v_basics();
    v_reference();
}

/*
* key note: dropping a vector drops its elements
* vectors can get different types in macros (if the exact size is given in the compile time)

*/
fn v_basics() {
    let v_ignore: Vec<i32> = Vec::new();
    let v_ignore2 = vec![1, 2, 3]; // * vec! is the macro to store given values in vector

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
}

fn v_reference() {
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];

    // v.push(6); - won't compile, because of the ownership principle.
    println!("The first element is: {}", first);
}
