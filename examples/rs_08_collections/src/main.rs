use std::collections::HashMap;

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // let v_with_type: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3, 4, 5];

    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("{}", i);
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    let s1 = String::from("hello");
    let s2 = String::from("world");
    let s1 = s1 + "-" + &s2;

    let astr = String::from("hahaha");

    for c in astr.chars() {
        println!("{}", c);
    }

    let vstr: Vec<&str> = astr.split('a').collect();

    let text = "  hello   world   wonderful   world  ";
    let v: Vec<&str> = text.split_whitespace().collect();
    println!("{:?}", v);

    let text = "hello world wonderful world";

    let mut mp = HashMap::new();
    for word in text.split_ascii_whitespace() {
        let count = mp.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:#?}", mp);

    let mut av = vec![5, 3, 1, 2, 4,7,8,1,4,2];

    av.select_nth_unstable(1);

    println!("{:?}", av);
}
