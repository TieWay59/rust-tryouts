
use std::{
    io,
    collections::HashMap,
    collections::HashSet,
};


#[derive(Debug)]
enum IP {
    V4(String),
    V6(String),
}

fn main() {
    let four = IP::V4(String::from("1.0.0.1"));
    let six = IP::V6(String::from("1.2.2.1"));
    
    println!("{:?} {:?}", four, six);

    let absent_number: Option<i32> = Some(5);

    if let IP::V4(s) = four {
        println!("{}", s);
    }

    match absent_number {
        Some(t) => println!("{}", t),
        _ => println!("error")
    }
    
    let mut books = HashSet::new();

    books.insert("hhh");
    books.insert("aaa");
    books.insert("cca");

    for (i, book) in books.iter().enumerate() {
        println!("{} {}", i, book);
    }

    let l = 0b_1000 << 20 ;

    println!("{}", l);


}
