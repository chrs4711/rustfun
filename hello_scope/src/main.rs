fn main() {

    println!("Hello, scope!");
    nothing_special(123);
    super_normal_number_stuff();
    
    string_fun();

    more_string_fun();
}

/// nothing special here
fn nothing_special(x: i32)  {
    println!("value: {}\n", x);
}

/// now we have _two_ variables of whatever the type of '5' is on the stack.
fn super_normal_number_stuff() {
    
    println!("no references, just values:");
    let mut x = 5;
    let y = x;
    
    println!("x: {}, y: {}", x, y);

    x = 2;
    println!("x: {}, y: {}\n", x, y);
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
    println!("Some string: {}\n", s2);
        
    // s1 is invalid, so we only have to free up the memory that s2 points to.
    // if we go out of scope, the special function 'drop' is called.
}

fn more_string_fun() {

    let s1 = String::from("foobar");

    print_string_length(&s1);

    println!("My String: {}\n", s1);

    let s1 = gimme_string();
    print_string_length(&s1);

    let s1 = process_string(s1);
    println!("s2: {}", s1);

    take_ownership(s1);

    // won't work!1
    // println!("s1: {}", s1);
}

/// The underscore tells the compiler not to worry about the unused variable
fn take_ownership(_s: String) {
    // we do nothing here :-D
}

fn print_string_length(s: &String) {
    println!("Lengh of '{}': {} bytes", s, s.len());
}

/// a function that creates a string for us
fn gimme_string() -> String {
    let some_string = String::from("hello");

    some_string // gets moved to the calling function
}

/// the function taketh, the function giveth
fn process_string(a_string: String) -> String {
    a_string
}
