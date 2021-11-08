struct Solution {}
impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let mut l: usize = 0;
        let mut v = i32::MAX;
        let mut ret = 0;
        for r in 1..nums.len() {
            let d = nums[r] - nums[r - 1];
            if d != v {
                l = r - 1;
                v = d;
            }
            ret += (r - l + 1) as i32 - 2;
        }
        ret
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::number_of_arithmetic_slices(vec![1, 2, 3, 4])
    );
}
