use std::io;

fn main() {
    println!("Hello, world!");
    another_function_1();
    another_function_2(3);
    print_labeled_measurement(3, 'c');

    let r :i32  = five();

    println!("{}", r);

    let rr = plus_one(99);
    println!("{rr}");

    //
    //
    // let cc: i32 =  1;
    // println!("{}", cc);
    // let y = {
    //     let x = 3;
    //     x + 1
    // };
    //
    // println!("The value of y is: {y}");
}

fn another_function_1() {
    println!("Another function.");
}

fn another_function_2(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}