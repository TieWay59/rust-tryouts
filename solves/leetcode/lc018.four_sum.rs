struct Solution {}

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if nums.len() < 4 {
            return vec![];
        }
        let mut ret = Vec::with_capacity(nums.len().pow(2));
        nums.sort();
        let cnt = nums
            .iter()
            .rev()
            .scan(std::collections::BTreeMap::new(), |s, ai| {
                *s.entry(*ai).or_insert(0) += 1;
                Some(s.clone())
            })
            .collect::<Vec<_>>()
            .iter()
            .rev()
            .cloned()
            .collect::<Vec<_>>();
        let a = &nums;

        for i in 0..a.len() - 3 {
            if i > 0 && a[i] == a[i - 1] {
                continue;
            }
            for j in i + 1..a.len() - 2 {
                if j > i + 1 && a[j] == a[j - 1] {
                    continue;
                }
                for k in j + 1..a.len() - 1 {
                    if k > j + 1 && a[k] == a[k - 1] {
                        continue;
                    }
                    let x = target - a[i] - a[j] - a[k];
                    if cnt[k + 1].contains_key(&x) {
                        ret.push(vec![a[i], a[j], a[k], x]);
                    }
                }
            }
        }
        ret
    }

    pub fn four_sum_2p(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut ret = Vec::with_capacity(nums.len().pow(2));
        if nums.len() < 4 {
            return ret;
        }
        nums.sort();
        let a = &nums;
        for i in 0..a.len() - 3 {
            if 0 < i && a[i] == a[i - 1] {
                continue;
            }
            for j in i + 1..a.len() - 2 {
                if i + 1 < j && a[j] == a[j - 1] {
                    continue;
                }
                let (mut l, mut r) = (j + 1, a.len() - 1);
                while l < r {
                    let sum = a[i] + a[j] + a[l] + a[r];
                    if sum == target {
                        ret.push(vec![a[i], a[j], a[l], a[r]]);
                        while l < r && a[l] == a[l + 1] {
                            l += 1;
                        }
                        while l < r && a[r] == a[r - 1] {
                            r -= 1;
                        }
                        l += 1;
                        r -= 1;
                    } else if sum < target {
                        l += 1;
                    } else if sum > target {
                        r -= 1;
                    }
                }
            }
        }
        ret
    }
    pub fn four_sum_set(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut s = std::collections::HashSet::with_capacity(nums.len());
        let mut ret = Vec::with_capacity(nums.len().pow(2));
        nums.sort();
        let a = &nums;
        if let Some(&item) = a.get(0) {
            s.insert(item);
        }
        for i in 1..a.len() {
            for j in i + 1..a.len() {
                for k in j + 1..a.len() {
                    if let Some(&x) = s.get(&(target - a[i] - a[j] - a[k])) {
                        ret.push(vec![x, a[i], a[j], a[k]]);
                    }
                }
            }
            s.insert(a[i]);
        }
        ret.sort();
        ret.dedup();
        ret
    }
}
fn main() {
    println!("{:?}", Solution::four_sum(vec![-2, -1, -1, 1, 1, 2, 2], 0));
    println!("{:?}", Solution::four_sum(vec![0], 0));
    println!("{:?}", Solution::four_sum(vec![2, 2, 2, 2, 2, 2, 2, 2], 8));
}
