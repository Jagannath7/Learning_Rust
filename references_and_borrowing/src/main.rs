fn main() {

    // reference example
    let mut s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}", s1, len);

    // change(&s1);

    mutable_change(&mut s1);
    
    println!("{}", s1);

    // Slice
    let s = String::from("hello world");
    let hello = &s[..5]; // same as [0..5]
    let world = &s[6..]; // same as [6..11]
    let whole_string = &s[..] // slice the whole string

}

fn calculate_length(s: &String) -> usize{ // s is a reference to a String
    return s.len()
} // s goes out of scope. Because it does not have any ownsership on s1, when it is out of scope, s1 is not dropped. 

// won't work. Trying to change a borrowed value.
// fn change(s: &String) {
//     s.push_str("blah blah!!");
// }

// MUTALBE REFERENCES

fn mutable_change(s: &mut String){
    s.push_str("blah blah!!");
}

// The Slice Type Reference!
fn first_word(s: &String) -> &str{
    let bytes = s.as_bytes();

    for(i, item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}
