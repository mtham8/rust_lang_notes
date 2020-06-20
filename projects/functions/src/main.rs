/*
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
*/
/*
fn five() -> i32 {
    5 // notice no semicolon bc expression
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}
*/
/*
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}
*/
/*
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
*/
fn main() {
    for number in (1..4).rev() {
        // reverse
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
