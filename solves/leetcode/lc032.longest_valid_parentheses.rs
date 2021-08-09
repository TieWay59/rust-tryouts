struct Solution {}

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut st = Vec::with_capacity(s.len());
        let mut cnt = 0;
        s.chars().fold(0, |mut n, c| {
            if c == '(' {
                st.push(cnt);
                cnt = 0;
            } else if let Some(val) = st.pop() {
                cnt = cnt + val + 2;
                n = n.max(cnt);
            } else {
                cnt = 0;
            }
            n
        })
    }
    pub fn longest_valid_parentheses_dp(s: String) -> i32 {
        let mut ret = 0;
        let mut v = vec![0 as usize; s.len()];
        for (i, c) in s.char_indices() {
            if i == 0 {
                continue;
            }
            let j = i - 1;
            if c == ')' && j >= v[j] && s.as_bytes()[j - v[j]] as char == '(' {
                v[i] = v[j] + 2 + v[j.saturating_sub(v[j] + 1)]; // v[0] 必定是 0
                ret = ret.max(v[i]);
            }
        }
        ret as i32
    }
    pub fn longest_valid_parentheses_first(s: String) -> i32 {
        let mut ret = 0;
        let mut v = vec![0 as usize; s.len()];
        for (i, c) in s.char_indices() {
            if c == '(' {
                v[i] = 0;
            } else if i > 0 {
                let j = i - 1;
                if j >= v[j] && s.as_bytes()[j - v[j]] as char == '(' {
                    v[i] = v[j] + 2 + *v.get(j.saturating_sub(v[j] + 1)).unwrap_or(&0);
                } else {
                    v[i] = 0;
                }
            }
            ret = ret.max(v[i]);
        }
        ret as i32
    }
}
fn main() {
    println!(
        "{:?}",
        Solution::longest_valid_parentheses("(()".to_string())
    );
    println!(
        "{:?}",
        Solution::longest_valid_parentheses(")()())".to_string())
    );
    println!("{:?}", Solution::longest_valid_parentheses("".to_string()));
    println!(
        "{:?}",
        Solution::longest_valid_parentheses("())".to_string())
    );
    println!(
        "{:?}",
        Solution::longest_valid_parentheses("()(()".to_string())
    );
    println!(
        "{:?}",
        Solution::longest_valid_parentheses("()(())".to_string())
    );
    println!(
        "{:?}",
        Solution::longest_valid_parentheses(")()())()()(".to_string())
    );
}
