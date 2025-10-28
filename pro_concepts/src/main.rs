fn main() {

    let mut x=5;
    println!("Hello, world! The value of x is: {}", x);
    x=90;
    println!("Hello, world! The value of x is: {}", x);

    //Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Constant value: {}", THREE_HOURS_IN_SECONDS);

    let alpha=50;
    println!("The value of alpha is: {}", alpha);
    {
    let alpha=alpha+10;
    println!("The value of alpha is: {}", alpha);
    }
    println!("The value of alpha is: {}", alpha);

    let mut spaces= "  ";
    spaces=spaces.len();
    println!("The number of spaces is: {}", spaces);
}
