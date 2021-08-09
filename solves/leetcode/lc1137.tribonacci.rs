struct Solution {}

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        (0..n).fold((0, 1, 1), |(x, y, z), _| (y, z, x + y + z)).0
    }
    pub fn tribonacci_naive(n: i32) -> i32 {
        let mut s: Vec<i32> = vec![-1; 38];
        fn gen(n: usize, s: &mut Vec<i32>) -> i32 {
            match n {
                0 => 0,
                1 => 1,
                2 => 1,
                _ => {
                    if s[n] == -1 {
                        s[n] = gen(n - 1, s) + gen(n - 2, s) + gen(n - 3, s);
                    }
                    s[n]
                }
            }
        }
        gen(n as usize, &mut s)
    }
}

fn main() {
    println!("{:?}", Solution::tribonacci(4));
    println!("{:?}", Solution::tribonacci(25));
}
