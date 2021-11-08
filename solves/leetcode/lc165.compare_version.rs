struct Solution {}
impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let a: Vec<&str> = version1.split('.').collect();
        let b: Vec<&str> = version2.split('.').collect();
        for i in 0..a.len().max(b.len()) {
            let id_1: i32 = String::from(*a.get(i).unwrap_or(&"0")).parse().unwrap();
            let id_2: i32 = String::from(*b.get(i).unwrap_or(&"0")).parse().unwrap();
            if id_1 > id_2 {
                return 1;
            }
            if id_1 < id_2 {
                return -1;
            }
        }
        0
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::compare_version("1.01".to_string(), "1.001".to_string())
    );
}
