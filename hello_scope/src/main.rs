fn main() {

    println!("Hello, scope!");
    nothing_special(123);
    super_normal_number_stuff();
    
    string_fun();

    more_string_fun();
}

/// nothing special here
fn nothing_special(x: i32)  {
    println!("value: {}", x);
}

/// now we have _two_ variables of whatever the type of '5' is on the stack.
fn super_normal_number_stuff() {
    let mut x = 5;
    let y = x;

    println!("x: {}, y: {}", x, y);

    // no references, just values.
    x = 2;
    println!("x: {}, y: {}", x, y);
}


fn string_fun() {
    
    // memory is allocated on the heap
    let s1 = String::from("foo");

    println!("Some string: {}", s1);

    // now, both s1 and s2 point to the same string object in memory
    let mut s2 = s1;

    // But it's forbidden now to access s1, it has become invalid!
    // println!("Some string: {}", s1); ERROR!!

    s2.push_str("sdfads");
    println!("Some string: {}", s2);
        
    // s1 is invalid, so we only have to free up the memory that s2 points to.
    // if we go out of scope, the special function 'drop' is called.
}

fn more_string_fun() {

    let s1 = String::from("foobar");

    print_string_length(&s1);

    println!("My String: {}", s1);
}

fn print_string_length(s: &String) {
    println!("Lengh of '{}': {} bytes", s, s.len());
}
