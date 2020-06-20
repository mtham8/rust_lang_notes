/*
fn main() {
    // s is not valid here, it's not declared
    let mut s = String::from("hello"); // s is valid from this point forward
    s.push_str(", world!");
    println!("{}", s); // do stuff with s
} // this scope is now over, and s is no longer valid

// when `s` comes into scope, it is valid
// it remains valid until it goes out of scope

// when you see a call to `clone`, you know that some arbitrary code is being executed and that code may be expensive. It's a visual indicator that something different is going on.

// if a type has the `Copy` trait, an older variable is still usable after assignment
// rust won't let us annotate a type with the `Copy` trait if the type, or any of its part, has implemented the `Drop` trait
*/

/*
fn main() {
    let s = String::from("hello"); // s comes into scope
    take_ownership(s); // s moved into function and s is not longer valid here

    let mut x = 5; // x comes into scope
    makes_copy(x); // x moved into function, but i32 is Copy, so it's still okay to use x afterward
    x += 1;
    println!("x: {}", x)
}

fn take_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string)
} // some_string goes out of scope and `drop` is called. backing memory is freed

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer)
} // some_integer goes out of scope, nothing happens
*/

/*
// mutable references
fn main() {
    let mut s = String::from("hello"); // you can have only one mutable reference to a particular piece of data in a particular scope
                                       // also cannot have a mutable reference while having an immutable one
                                       // multiple immutable references are okay bc no one who is reading the data has the ability to change the data others are reading
    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
*/

// dangling pointers
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    // dangle returns a reference to a String
    let s = String::from("hello"); // s is a new String
    &s // return a reference to the String, s
} // s goes out of scope, and is dropped. memory goes away - danger!

// solution: return the `String` directly - ownership is moved out, and nothing is deallocated

// rules of references
// at any given time, you can have either one mutable reference or any number of immutable references
// references must always be valid
