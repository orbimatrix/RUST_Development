fn main() {
    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

    let ss = String::from("hello");  // s comes into scope

    takes_ownership(ss);             // ss's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);  

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    let some_data=String::from("Hello");

    let some_value=some_data.clone();

    
    println!("some_data = {some_data}, some_value = {some_value}");

}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}