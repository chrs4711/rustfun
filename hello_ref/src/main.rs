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

    let s = String::from("foo bar");
    first_word_index(&s);
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

fn _better() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} {}", r1, r2);

    // nobody uses a pointer to s beyond this part, so nobody will
    // be surprised by changing data. I.e. now a mutable ref is ok. 

    let r3 = &mut s;
    println!("{}", r3);
}

/// the same without references. It steals ownership
/// of the passed string.
fn _calculate_length_awkward(s: String) -> usize {
    s.len()
}

fn first_word_index(s: &String) -> usize {

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        print!("--> checking [{}]", item);
        if item == b' ' {
            println!(" HIT");
            return i;
        }
        println!();
    }

    s.len()
}

fn first_word_sl(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
