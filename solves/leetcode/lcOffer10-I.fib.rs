struct Solution {}
impl Solution {
    pub fn fib(n: i32) -> i32 {
        (0..n).fold((0, 1), |(i, j), _| (j, (i + j) % 1000000007)).0
    }

    pub fn fib_naive(n: i32) -> i32 {
        fn f((i, j): (i32, i32), k: i32, m: i32) -> i32 {
            if k > 0 {
                return f((j, (i + j) % m), k - 1, m);
            }
            i
        }
        f((0, 1), n, 1000000007)
    }
}
fn main() {
    println!("{:?}", Solution::fib(2));
    println!("{:?}", Solution::fib(5));
}
