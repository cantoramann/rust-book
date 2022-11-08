fn main() {
    basic_reference_example();
    modify_mutable_string();
    wrong();
}

fn basic_reference_example() {
    println!("Getting string length with a reference - so we get the length and keep the string");
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
    println!("\n");
}

fn calculate_length(s: &String) -> usize {
    // ! s is a reference to a String. Hence, this func does not own s
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

fn modify_mutable_string() {
    println!("Modifying a mutable string");

    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);

    println!("\n");
}

fn change(some_string: &mut String) {
    some_string.push_str(", world"); // this won't work if the passed in string was not mutable
}

fn wrong() {
    /*
     ! restriction: if you have a mutable reference to a value, you cannot have any other
     ! reference to that value (regardless of mutable or not). So, one mutable and one immutable
     ! refrence to the samue value also does not compile.
    */
    // let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);

    // * but this part works - because r1 reference goes out of scope after the inner expression
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;

    /*
     * below works only because r1 and r2 are not used after r3. If they were, this would fail.
     */
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
    // println!("{}", r1); - won't compile, because if r1 is used after the declaration of the r3 reference,
    // ! then the ownership principle will be broken. If there is a mutable reference, no other reference may exist
    // ! or they shall not be used afterwards.
}

fn dangling_reference() {
    /*
     * in Rust, there is no possibility of having a dangling reference!
     */

    // let reference_to_nothing = dangle();
    let reference_to_copy = no_dangle();
}

/*
fn dangle() -> &String {
    /*
    This is a dangling reference because a dropped string reference is returned.
    ! This will give a compilation error and fail
    */
    // let s = String::from("hello");

    // &s
}
 */

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
