struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut s = s;
        while s.contains("()") || s.contains("[]") || s.contains("{}") {
            s = s.replace("()", "").replace("[]", "").replace("{}", "");
        }
        s.len() == 0
    }
    pub fn is_valid_refined(s: String) -> bool {
        fn check(s: String) -> Option<bool> {
            let mut v = Vec::with_capacity(s.len());
            for ai in s.chars() {
                match ai {
                    '(' | '[' | '{' => v.push(ai),
                    _ => match (v.pop()?, ai) {
                        ('(', ')') => continue,
                        ('[', ']') => continue,
                        ('{', '}') => continue,
                        _ => return Some(false),
                    },
                }
            }
            Some(v.len() == 0)
        }
        check(s).unwrap_or(false)
    }
    pub fn is_valid_naive(s: String) -> bool {
        fn check(s: String) -> Option<bool> {
            let mut v = Vec::with_capacity(s.len());
            for ai in s.chars() {
                if ai == '(' || ai == '[' || ai == '{' {
                    v.push(ai);
                } else {
                    match (v.pop()?, ai) {
                        ('(', ')') => continue,
                        ('[', ']') => continue,
                        ('{', '}') => continue,
                        _ => return None,
                    }
                }
            }
            if v.len() > 0 {
                return None;
            }
            Some(true)
        }
        check(s).unwrap_or(false)
    }
}
fn main() {
    println!("{:?}", Solution::is_valid("()".to_string()));
    println!("{:?}", Solution::is_valid("()[]{}".to_string()));
    println!("{:?}", Solution::is_valid("(]".to_string()));
    println!("{:?}", Solution::is_valid("([)]".to_string()));
    println!("{:?}", Solution::is_valid("{[]}".to_string()));
}
