struct Solution {}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        fn gen(ret: &mut Vec<String>, n: i32, l: i32, r: i32, s: &mut String) {
            if l + r == n * 2 {
                if l == r {
                    return ret.push(s.clone());
                } else {
                    return;
                }
            }
            if l < n {
                s.push('(');
                gen(ret, n, l + 1, r, s);
                s.pop();
            }
            if l > r {
                s.push(')');
                gen(ret, n, l, r + 1, s);
                s.pop();
            }
        }
        let mut ret = Vec::new();
        gen(
            &mut ret,
            n,
            0,
            0,
            &mut String::with_capacity(2 * n as usize),
        );
        ret
    }
    pub fn generate_parenthesis_naive(n: i32) -> Vec<String> {
        let mut ret = Vec::new();
        fn gen(ret: &mut Vec<String>, n: usize, l: usize, r: usize, s: String) {
            if l + r == n * 2 {
                if l == r {
                    return ret.push(s);
                } else {
                    return;
                }
            }
            if l < n {
                gen(ret, n, l + 1, r, s.clone() + "(");
            }
            if l > r {
                gen(ret, n, l, r + 1, s + ")");
            }
        }
        gen(&mut ret, n as usize, 0, 0, String::from(""));
        ret
    }
}
fn main() {
    println!("{:?}", Solution::generate_parenthesis(5));
}
