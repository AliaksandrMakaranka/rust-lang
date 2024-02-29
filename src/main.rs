
fn main() {
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    //
    // x = 6;
    // println!("The value of x is: {}", x);
    //
    //     const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // println!("{}", THREE_HOURS_IN_SECONDS);

    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of x in the inner scope is: {y}");
    }

    println!("The value of x is: {y}");

    let spaces = "q";
    let spaces = spaces.len();
    println!("f: {}", spaces);


    let mut spaces = "some";
    let spaces = spaces.len();

    println!("f2: {}", spaces);

}

