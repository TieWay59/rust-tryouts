const MAX_POINTS: u32 = 500_000;

fn main() {
    println!("The value of x is: {}", MAX_POINTS);

    let x: u8 = 0b_1000_0001;

    let y: u8 = 0b_0000_0101;

    let y = y + 5;

    println!("{} {:b} ", x, y);

    let a_tuple = (1, 2, 3);

    println!("{} {} {}", a_tuple.0, a_tuple.1, a_tuple.2);

    // ---------------------
    let mut arr = [3; 5];

    arr[0] = 1;

    for ai in arr {
        print!("{} ", ai);
    }
    println!();

    // ---------------------
    let a = [1, 2, 3, 4, 5];
    let big_index = 1;
    println!("{}", a[big_index]);

    // for i in 1..10 {
    //     println!("{}", a[i + 5]);
    // }

    // ---------------------
    println!("{}", gcd(16, 12));
    println!("{}", gcd(16, 18));

    println!("{}", fpow(2, 4, 1000));

    // ---------------------

    for n in (1..4).rev() {
        println!("- {}!", n);
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}

fn fpow(mut a: u64, mut b: u64, m: u64) -> u64 {
    let mut ret = 1;
    while b > 0 {
        if (b & 1) > 0 {
            ret = ret * a % m;
        }
        b = b >> 1;
        a = a * a;
    }
    ret
}
