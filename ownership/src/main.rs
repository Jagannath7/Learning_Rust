fn main() {
    // // s is not valid here
    // {
    //     let s = "hello"; // s is valid here
    // }
    // // s is not valid here because the scope of s is over.

    // // String data type -> stored on the heap.

    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s);

    let x = 5;
    let y = x;

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("{}",s1);

    let my_string = String::from("hello");
    takes_ownership(my_string);

    let x = 5;
    makes_copy(x);

    


}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.