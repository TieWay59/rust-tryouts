const fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    let mut fib = [0u64; 50];

    for i in 0..50 {
        fib[i] = match i {
            0 => 1,
            1 => 1,
            _ => fib[i - 1] + fib[i - 2],
        }
    }

    println!("{}", fib[30]);
    println!("{}", fibonacci(50));
}
