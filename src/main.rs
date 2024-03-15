fn main() {
    //
    // let mut s = String::from("hello");
    // let ss = "someText";
    // println!("{}", ss);
    //
    // s.push_str(", world!");
    // println!("{}",s);
    //
    // let x = 5;
    // let y = x;
    //
    // println!("{}, {}", x, y);
    //
    // let s1 = String::from("hello");
    // let s2 = s1.clone();
    //
    // println!("{}, world!", s2);

    println!("{}", "***************************************");

    // let s = String::from("hello");  // s comes into scope

    // takes_ownership(s);             // s's value moves into the function...
    // ... and so is no longer valid here

    // let x = 5;                      // x comes into scope

    // makes_copy(x);                  // x would move into the function,
    // but i32 is Copy, so it's okay to still
    // use x afterward
    // let s1 = gives_ownership();         // gives_ownership moves its return
    // value into s1

    // let s2 = String::from("hello");     // s2 comes into scope

    // let s3 = takes_and_gives_back();

    // println!("{}", s1);
    // println!("{}", s2);
    // println!("{}", s3);

    // let s1 = String::from("hello");
    //
    // let (s2, len) = calculate_length(s1);
    //
    // println!("The length of '{}' is {}.", s2, len);
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}", r1, r2);

    let r3 = &mut s; // no problem
    println!("{}", r3);

    let g = dangle();
    print!("{}", g);
}

fn dangle() -> String {
    let s = String::from("dangle");
    s
}

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len(); // len() returns the length of a String
//     // (1.to_string(), 2);
//     (s, length)
// }


// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// fn gives_ownership() -> String {             // gives_ownership will move its
//     // return value into the function
//     // that calls it
//
//     let some_string = String::from("yours"); // some_string comes into scope
//
//     some_string                              // some_string is returned and
//     // moves out to the calling
//     // function
// }

// This function takes a String and returns one
// fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
//     // scope
//
//     a_string  // a_string is returned and moves out to the calling function
// }

//
// fn takes_ownership(ss: String) { // some_string comes into scope
//     println!("{}", ss);
// } // Here, some_string goes out of scope and `drop` is called. The backing
// // memory is freed.
//
// fn makes_copy(sint: i32) { // some_integer comes into scope
//     println!("{}", sint);
// } // Here, some_integer goes out of scope. Nothing special happens.