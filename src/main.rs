use std::io;

fn main() {
    let guess: u128 = "123".parse().expect("Not a number!");
    println!("{}", guess);

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    let a = 2.0f64; // f64

    let b = 3.0f32; // f32

    println!("f64: x + a = {}", x + a);
    println!("f32: y + b = {}", y + b);

    println!("{}", "_________________________");

    // addition
    let sum = 5 + 10;
    println!("sum {}", sum);

    // subtraction
    let difference = 3.5 - 4.3;
    println!("subtraction {}", difference);

    // multiplication
    let product = 0.1 * 10.0;
    println!("multiplication {}", product);

    // division
    let quotient = 3.0 / 4.0;
    println!("division quotient {}", quotient);
    let truncated = -9 / 3; // Results in -3
    println!("division truncated {}", truncated);

    // remainder
    let remainder = 16 % 5;
    println!("remainder {}", remainder);

    let t = true;
    let f: bool = false;
    println!("{}", t);
    println!("{}", f);

    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{}", c);
    println!("{}", z);
    println!("{}", heart_eyed_cat);

    let tup: (i32, f64, u8, &str) = (500, 6.4, 1, "f");
    println!("tup: {:#?}", tup);


    let tup1 = (500, 6.4, 1);

    let (x, y, z) = tup1;

    println!("The value of tup1: {y}, {z}, {:#?}", tup1);


    let x: (i32, f64, u8) = (500, 6.4, 1);
    println!("print x: {:?}", x);

    let five_hundred = x.0;
    println!("five_hundred: {}", five_hundred);

    let six_point_four = x.1;
    println!("six_point_four: {}", six_point_four);

    let one = x.2;
    println!("one: {}", one);

    let p = (1, 2, 3.4);
    println!("{:?}", p);

    let cot = ();
    println!("{:?}", cot);


    let a = [1, 2, 3, 4, 5];

    let arr = [-0.3, 4.3, 2.0];
    println!("last value in arr[] {:?}", arr[(arr.len()) - 1]);
    println!("{:?}", arr);

    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    println!("{:?}", months);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [4];
    let ax = [1.1; 15];

    println!("{:?}", b);
    println!("{:?}", a);
    println!("{:?}", ax);


    let a = [1, 2, 3, 4, 5];
    println!("{}", "*******************************");

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

