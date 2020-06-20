#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // methods can take ownership of `self`, borrow `self` immutably, or borrow `self` mutably
    // if we wanted to change the instance that we've called the method on as part of what it does, we'd use `&mut self` as the first parameter
    // for area(), we use `&self` bc the method is reading the instance
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    //println!("rect1 is {:?}", rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    )
}

/*
// we want to borrow the struct rather than take ownership of it. this way `main` retains its ownership and can continue using `rect1`, which is the reason why we use the `&` in the function signature and where we call the function
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
*/
// we use structs to add meaning by labeling the data
