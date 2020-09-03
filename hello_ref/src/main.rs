use std::io::prelude::*;
use std::io;
use std::{thread, time};

fn main() {
    println!("Fun with references!");

    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}", s1, len);
    
    string_fun();

    loop_fun();
    while_fun();
    for_fun();
}

fn string_fun() {
    println!("\n### string_fun!");

    // If we want to change stuff around in functions, more ceremony is needed:
    let mut s2 = String::from("changeme");
    println!("String: '{}'", s2);
    modify_string(&mut s2);
    println!("String: '{}'", s2);

    let s = String::from("foo bar");
    first_word_index(&s);

    let s2 = String::from("hello world");
    let str = first_word_sl(&s2);
    println!("first word of '{}' is '{}'", &s2, str);

    println!("first word of '{}' is '{}'", "foo bar", first_word_sl("foo bar"));

    // So string slice is compatible to String or what?
    println!("first word of '{}' is '{}'", "foo bar", first_word_sl(&String::from("foo bar")));
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn modify_string(s: &mut String) {
    s.push_str("trololo");
}

fn _wont_work() {
    let mut s = String::from("hello");

    let _r1 = &s; // super nice read-only references
    let _r2 = &s; // super nice read-only references
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

/// Apparently we can also plug a reference to a String into that ¯\_(ツ)_/¯
fn first_word_sl(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn loop_fun() {
    
    println!("\n### loop_fun!");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // hehe that's the return value
        }
    };

    println!("The result is {}", result);
}

fn while_fun() {

    println!("\n### while_fun!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        print!("[{}]", a[index]);

        index += 1;
    }

    println!();
}

fn for_fun() {

    println!("\n### for_fun!");

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        print!("[{}]", element);
    }

    println!();

    for number in (1..10).rev() {
        print!("{}..", number);
        io::stdout().flush().ok().expect("wtf");
        thread::sleep(time::Duration::from_millis(500));
    }

    println!("LIFTOFF!!!1");
}