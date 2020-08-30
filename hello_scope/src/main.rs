fn main() {

    println!("Hello, scope!");
    nothing_special(123);
}

/// nothing special here
fn nothing_special(x: i32)  {
    println!("value: {}", x);
}
