struct Solution {}

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        for i in (0..nums.len() - 1).rev() {
            if nums[i] < nums[i + 1] {
                nums[i + 1..].reverse();
                // let j = i + 1 + nums[i + 1..].partition_point(|&x| x <= nums[i]); stable 1.52.0
                // nums.swap(i, j);
                for j in i + 1..nums.len() {
                    if nums[i] < nums[j] {
                        nums.swap(i, j);
                        break;
                    }
                }
                // println!("{:?}", nums);
                return;
            }
        }
        nums.reverse();
    }
    pub fn next_permutation_naive(nums: &mut Vec<i32>) {
        let p1 = nums.len() - 1;
        let mut p2 = p1;
        let n = nums.len();
        for i in (0..n - 1).rev() {
            if nums[i] < nums[i + 1] {
                p2 = i;
                break;
            }
        }
        if p2 < p1 {
            let mut i = p2 + 1;
            let mut j = p1;
            while i < j {
                nums.swap(i, j);
                i += 1;
                j -= 1;
            }
            for i in p2 + 1..=p1 {
                if nums[p2] < nums[i] {
                    nums.swap(p2, i);
                    break;
                }
            }
        } else {
            nums.sort();
        }
        println!("{:?}", nums);
    }
}

fn main() {
    println!("{:?}", Solution::next_permutation(&mut vec![1, 2, 3]));
    println!("{:?}", Solution::next_permutation(&mut vec![3, 2, 1]));
    println!("{:?}", Solution::next_permutation(&mut vec![1, 1, 5]));
    println!("{:?}", Solution::next_permutation(&mut vec![1, 5, 1]));
    println!(
        "{:?}",
        Solution::next_permutation(&mut vec![2, 1, 2, 2, 2, 2, 2, 1])
    );
    println!("{:?}", Solution::next_permutation(&mut vec![1]));
}
