struct Solution {}
impl Solution {
    pub fn smallest_k(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let mut v = arr.clone();
        v.sort();
        v.truncate(k as usize);
        v
    }
}
fn main() {
    println!(
        "{:?}",
        Solution::smallest_k(vec![1, 3, 5, 7, 2, 4, 6, 8], 4)
    );
}
