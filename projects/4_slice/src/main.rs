// slice
// slice lets you reference a contiguous sequence of elements in a collection rather than the whole collection
// does not have ownership

fn main() {
    let mut s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    let word = first_word(&s); // word will get the value s
    s.clear(); // this empties the String, making it equal to ""
               // word still has the value 5 here, but there's no more string that
               // we could meaningfully use the value 5 with. word is now totally invalid!
}

fn first_word(s: &str) -> &str {
    // &str = string slice
    // convert string to array of bytes
    let bytes = s.as_bytes();
    // iter returns each element, enumerate wraps the results of iter and return each element as part of a tuple instead
    // we get a reference to the element from .iter().enumerate()
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// string literals are slices = &string = immutable reference
// if we have a string slice, we can pass that directly. if we have a `String`, we can pass a slice of the entire string
// defining a function to take a string slice instead of a reference to a `String` makes our API more general and useful without losing functionality
