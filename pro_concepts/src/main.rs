use std::io;

fn main() {

// dummy_function();
let x=5;
print_value(x);
add_(x);
let result=rectangle_area(10,20);
println!("The area of the rectangle is: {}", result);
let five=fiver();
println!("The value returned by fiver is: {}", five);

}

fn add_(x:i32) {
    println!("The value of x is: {}", x+10);
}

fn fiver()->i32{
    5
}

fn rectangle_area(width:i32,height:i32)->i32{
    width*height
}

fn print_value(x:i32){
    println!("The value of x is: {}", x);
}

fn dummy_function(){
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

    // let mut spaces= "  ";
    // spaces=spaces.len();
    // println!("The number of spaces is: {}", spaces);

    let guess:u32="42".parse().expect("Not a number!");
    println!("The value of guess is: {}", guess);

    let floating_point:f64 =4.5;

    let t=true;
    let f:bool =false;

    let c='z';
    let charac: char='e';
    println!("The characters are: {} and {}", c, charac);

    let tup:(i32,f64,u8)=(1200,6.4,12);
    let (x,y,z)=tup;

    println!("The value of y is: {}", y);

    let five_hundred= tup.0;
    println!("The value of five_hundred is: {}", five_hundred);

    let a:[i32;5]=[1,2,3,4,5];
    let first=a[0];
    let second=a[1];
    println!("The first element is: {}, and the second element is: {}", first, second);

    let array_data=[3;10];
    println!("The array data is: {:?}", array_data);

    let mut index=String::new();

    io::stdin().read_line(&mut index).expect("Failed to read line");    
    let index:usize=index.trim().parse().expect("Index not a number");  

    let element=array_data[index];
    println!("The element at index {} is: {}", index, element);



}