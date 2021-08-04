struct Solution {}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        fn rev(mut x: i32) -> Option<i32> {
            let mut ret = 0i32;
            while x != 0 {
                ret = ret.checked_mul(10)?.checked_add(x % 10)?;
                x = x / 10;
            }
            Some(ret)
        }
        rev(x).unwrap_or(0)
    }
    pub fn reverse_naive(x: i32) -> i32 {
        if x == 0 {
            return 0;
        }
        let x = x as i64; // 先提前现式强转
        let mut a = x.abs();
        let mut ret = 0;
        while a > 0 {
            ret = ret * 10 + a % 10;
            a /= 10;
        }
        ret = ret * (x / x.abs());
        if ret > i32::MAX as i64 || ret < i32::MIN as i64 {
            return 0;
        }
        ret as i32
    }
}
fn main() {
    println!("{:?}", Solution::reverse(-2147483648));
    println!("{:?}", Solution::reverse(123));
    println!("{:?}", Solution::reverse(-123));
    println!("{:?}", Solution::reverse(120));
    println!("{:?}", Solution::reverse(0));
}
