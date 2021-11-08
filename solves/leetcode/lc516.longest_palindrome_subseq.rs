struct Solution {}
impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let n = s.len();
        let s: Vec<_> = s.chars().collect();
        let mut dp = vec![vec![0i32; n]; n];
        for l in (0..n).rev() {
            dp[l][l] = 1;
            for r in l + 1..n {
                if s[l] == s[r] {
                    dp[l][r] = dp[l][r].max(dp[l + 1][r - 1] + 2);
                } else {
                    dp[l][r] = dp[l][r - 1].max(dp[l + 1][r]);
                }
            }
        }
        dp[0][n - 1]
    }
    pub fn longest_palindrome_subseq_naive(s: String) -> i32 {
        let n = s.len();
        let s: Vec<_> = s.chars().collect();
        let mut dp = vec![vec![0i32; n + 1]; n];
        for i in 0..n {
            dp[i][1] = 1;
        }
        for l in 2..=n {
            for i in 0..n - l + 1 {
                if s[i] == s[i + l - 1] {
                    dp[i][l] = dp[i][l].max(dp[i + 1][l - 2] + 2);
                } else {
                    dp[i][l] = dp[i][l - 1].max(dp[i + 1][l - 1]);
                }
            }
        }
        dp[0][n]
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::longest_palindrome_subseq("bbbab".to_string())
    );
    println!(
        "{:?}",
        Solution::longest_palindrome_subseq("cbbd".to_string())
    );
}
