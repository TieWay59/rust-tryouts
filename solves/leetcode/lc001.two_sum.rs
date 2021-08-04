struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut mp = std::collections::HashMap::with_capacity(nums.len());
        nums.iter()
            .enumerate()
            .fold(Vec::new(), |mut state, (i, n)| {
                if let Some(&j) = mp.get(&(target - n)) {
                    state = vec![j as i32, i as i32];
                }
                mp.insert(n, i);
                state
            })
    }
    
}

fn main() {
    println!("{:?}", Solution::two_sum(vec![3, 2, 4], 6));
}
