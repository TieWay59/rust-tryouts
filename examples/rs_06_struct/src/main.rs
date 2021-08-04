
#[derive(Debug)]
struct User {
    name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// struct Color(u32, u32, u32);

// struct Point(i32, i32);

struct Rec {
    h: u32,
    w: u32,
}

impl Rec {
    fn area(&self) -> u32 {
        self.w * self.h
    }
    fn square(size: u32) -> Rec {
        Rec {w: size, h:size}
    }
}

fn main() {

    let user1 = User {
        email: String::from("hhhh"),
        name: String::from("Alice"),
        active: true,
        sign_in_count: 1,
    };

    let rec = Rec { h: 10, w: 20};
    
    let square = Rec::square(20);

    println!("Hello, {:#?}!", user1);
    println!("area: {}", rec.area());
    println!("area: {}", square.area());
}
