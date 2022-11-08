struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    dec_struct();
    build_user_caller();
    build_user_from_other_users();
    colors_points();
    empty_struct();
    // struct_data_ownership(); // ! uncomment to see why this fails: expected named lifetime parameter. More below.
}

fn dec_struct() {
    println!("Declaring a struct.");
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    /*
    if we want to modify the struct later -> declare it as a mutable object (let mut user1)
    */
    user1.email = String::from("anotheremail@example.com");
    println!("\n");
}

fn build_user_caller() {
    println!("Build user caller.");

    let user1 = build_user(String::from("name"), String::from("username"));
    println!("\n");
}

/*
factory-like approach to generate structs. Keep in mind that this is not a reference.
*/
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn build_user_from_other_users() {
    println!("Build user from other users.");

    // let mut user1 = build_user(String::from("name"), String::from("username"));
    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    // // shadows user2

    let mut user1 = build_user(String::from("name"), String::from("username"));

    user1.email = String::from("user1@example.com");

    let user2 = User {
        email: String::from("user2@example.com"),
        ..user1
    };

    let user3 = User {
        email: String::from("user2@example.com"),
        ..user2 // ! if you change this with user1 - it won't compile. Looks like
                // ! once a value is moved, it won't move anymore. Change user2 with user1
                // ! and read the error message
    };

    println!("{}", user1.email);
    println!("{}", user2.email);
    println!("{}", user3.email);

    println!("\n");
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn colors_points() {
    println!("Color and point structs");
    // * Color and Point are different types, even if their member data are the same.

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("\n");
}

struct AlwaysEqual;

fn empty_struct() {
    println!("Empty struct");

    let subject = AlwaysEqual;

    println!("\n");
}

// struct UserReference { // ! uncomment this to see the error. Will be fixed in later chapters
//     active: bool,
//     username: &str, // this is different thatn the User struct above - an &str, not a String
//     email: &str,    // this is different thatn the User struct above - an &str, not a String
//     sign_in_count: u64,
// }

fn struct_data_ownership() {
    println!("Struct data ownership");
    /*
    * Storing a reference to another data in the struct. Yes, this is possible.

    * "It’s also possible for structs to store references to data
    * owned by something else, but to do so requires the use of lifetimes."
    * More on lifetimes later. For now just stick.
    */

    /*
    let user1 = UserReference {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    }; - won't compile because we need a string reference this time. Not the string itself.
    */

    println!("\n");
}
