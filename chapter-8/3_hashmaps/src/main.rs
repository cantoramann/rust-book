fn main() {
    read();
    ownership();
    update_based_on_old_value();
}

/*
* hashmaps are stored on the heap
* By default, HashMap uses a hashing function called SipHash that
* can provide resistance to Denial of Service (DoS) attacks involving hash tables
*/
fn intro() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    // scores.insert("Red", 50); - won't compile
}

fn read() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    /*
     * copied: turns the reference into value
     * unwrap_or: if does not exist, add 0 and return
     * without these two functions, get() will return an Option<&i32>.
     * now it returns an i32 (not a reference!)
     */
    println!("{}", score);
}

fn iterate() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

fn ownership() {
    /*
     * for types that implement the Copy trait (trivial types), no worries.
     * in that case, everything inside the map will be a copy.
     * for complex types (probably just the types on the heap like String),
     * things get a bit more complex. Hashmap owns them once inserted.
     */

    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    /*
    ! after this moment, field_name and field_value are invalid.
    ! they are invalidated.
    */

    // println!("{}", field_name); - won't compile
}

fn add_if_key_does_not_exist() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}

fn update_based_on_old_value() {
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // * here we do not get the value by copy. This may be important.
        *count += 1;
    }

    println!("{:?}", map);
}
