use std::cmp::Reverse;
struct Solution {}
impl Solution {
    // N * M
    pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
        let mut a = vec![i32::MAX; n as usize];
        let mut b: Vec<_> = primes.iter().map(|&p| (p, 0)).collect();

        a[0] = 1;
        for i in 1..n as usize {
            for &(v, _) in b.iter() {
                a[i] = a[i].min(v);
            }
            for (j, (v, k)) in b.iter_mut().enumerate() {
                if a[i] == *v {
                    *k += 1;
                    *v = a[*k] * primes[j];
                }
            }
        }
        a[n as usize - 1]
    }
    // 不明白为什么这么快！
    pub fn nth_super_ugly_number_nm_100p(n: i32, primes: Vec<i32>) -> i32 {
        let mut a = vec![1i32; n as usize];
        let mut b: Vec<_> = primes.iter().map(|&p| (p, 0, p)).collect();

        for i in 1..n as usize {
            let mut minv = i32::MAX;
            for &(v, _, _) in b.iter() {
                minv = v.min(minv);
            }
            a[i] = minv;
            for (v, j, p) in b.iter_mut() {
                if minv == *v {
                    *v = a[*j as usize + 1] * *p;
                    *j += 1;
                }
            }
        }
        a[a.len() - 1]
    }
    pub fn nth_super_ugly_number_nmlogm(n: i32, primes: Vec<i32>) -> i32 {
        let mut set = std::collections::BinaryHeap::new();
        let mut a = vec![1i32];
        for p in primes {
            set.push((Reverse(p), 0, p));
        }
        for _ in 1..n {
            while let Some((Reverse(v), i, p)) = set.pop() {
                if v != a[a.len() - 1] {
                    a.push(v);
                    if let Some(m) = p.checked_mul(a[i + 1]) {
                        set.push((Reverse(m), i + 1, p));
                    }
                    break;
                }
                if let Some(m) = p.checked_mul(a[i + 1]) {
                    set.push((Reverse(m), i + 1, p));
                }
            }
        }
        a[a.len() - 1]
    }
    pub fn nth_super_ugly_number_50ms(n: i32, primes: Vec<i32>) -> i32 {
        let mut set = std::collections::BinaryHeap::new();
        let mut seen = std::collections::HashSet::new();
        set.push(Reverse(1i32));
        seen.insert(1);
        let mut ret = 1;
        for _ in 0..n {
            if let Some(Reverse(v)) = set.pop() {
                ret = v;
            }
            for p in &primes {
                if let Some(val) = p.checked_mul(ret) {
                    if !seen.contains(&val) {
                        set.push(Reverse(val));
                        seen.insert(val);
                    }
                } else {
                    break;
                }
            }
        }
        ret
    }
    pub fn nth_super_ugly_number_naive(n: i32, primes: Vec<i32>) -> i32 {
        let mut set = std::collections::BTreeSet::new();
        set.insert(1);
        let mut ret = 1;
        for _ in 0..n {
            if let Some(&v) = set.iter().next() {
                ret = v;
            }
            set.remove(&ret);

            for p in &primes {
                if let Some(val) = p.checked_mul(ret) {
                    set.insert(val);
                } else {
                    break;
                }
            }
        }
        println!("len = {}", set.len());
        ret
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::nth_super_ugly_number(12, vec![2, 7, 13, 19])
    );
    println!("{:?}", Solution::nth_super_ugly_number(1, vec![2, 3, 5]));
    println!(
        "{:?}",
        Solution::nth_super_ugly_number(15, vec![3, 5, 7, 11, 19, 23, 29, 41, 43, 47])
    );
    // println!(
    //     "{:?}",
    //     Solution::nth_super_ugly_number(1000000, vec![2, 7, 13, 19])
    // );
    // println!(
    //     "{:?}",
    //     Solution::nth_super_ugly_number(1000000, vec![2, 3, 5])
    // );
    // println!("{:?}", Solution::nth_super_ugly_number(1000000, vec![2]));
}
