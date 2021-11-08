struct Solution {}
impl Solution {
    pub fn escape_ghosts(ghosts: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        let (x, y) = (target[0], target[1]);
        let mut d = x.abs() + y.abs();
        ghosts.iter().all(|e| (e[0] - x).abs() + (e[1]-y).abs() > d)
    }
    pub fn escape_ghosts_naive(ghosts: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        let (x, y) = (target[0], target[1]);
        let mut d = i32::MAX;
        for a in ghosts {
            let t = (x - a[0]).abs() + (y - a[1]).abs();
            d = d.min(t);
        }
        d > x.abs() + y.abs()
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::escape_ghosts(vec![vec![1, 0], vec![0, 3]], vec![0, 1])
    );
    println!(
        "{:?}",
        Solution::escape_ghosts(vec![vec![1, 0]], vec![2, 0])
    );
    println!(
        "{:?}",
        Solution::escape_ghosts(vec![vec![1, 0], vec![0, 3]], vec![0, 1])
    );
}
