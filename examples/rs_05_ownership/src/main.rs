use std::io;


fn get_first_word(s: &String) -> &str {
    let bs = s.as_bytes();
    
    for (i, &item) in bs.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn main() {
    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{} {}", s1, s2);

    let mut s = String::new();

    io::stdin().read_line(&mut s).expect("err");
    
    let word = get_first_word(&s);
    
    println!("={}", word);
    
    s.clear();

}
