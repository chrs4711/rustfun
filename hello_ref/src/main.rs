fn main() {
    println!("Fun with references!");

    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}", s1, len);
    
    // If we want to change stuff around in functions, more ceremony is needed:
    let mut s2 = String::from("changeme");
    println!("String: '{}'", s2);
    modify_string(&mut s2);
    println!("String: '{}'", s2);

}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn modify_string(s: &mut String) {
    s.push_str("trololo");
}

fn _wont_work() {
    let mut s = String::from("hello");

    let r1 = &s; // super nice read-only references
    let r2 = &s; // super nice read-only references
    let r3 = &mut s;

    // println!("{} {} {}", r1, r2, r3);
    //                               ^  NOPE!!
    // no borrowing as mutable when immutable references are passed around

    // That's cool however:
    println!("{}", r3);
}

/// the same without references. It steals ownership
/// of the passed string.
fn _calculate_length_awkward(s: String) -> usize {
    s.len()
}

