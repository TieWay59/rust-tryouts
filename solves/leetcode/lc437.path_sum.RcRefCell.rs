// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution {}

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        fn dfs(
            cur: &Option<Rc<RefCell<TreeNode>>>,
            mp: &mut BTreeMap<i32, i32>,
            sum: i32,
            target_sum: i32,
        ) -> i32 {
            if let Some(node) = cur {
                let n = node.borrow();
                let nsum = sum + n.val;
                let mut ret = *mp.get(&(nsum - target_sum)).unwrap_or(&0);
                mp.entry(nsum).and_modify(|e| *e += 1).or_insert(1);
                ret += dfs(&n.left, mp, nsum, target_sum);
                ret += dfs(&n.right, mp, nsum, target_sum);
                mp.entry(nsum).and_modify(|e| *e -= 1).or_default();
                return ret;
            }
            0
        }
        let mut mp = BTreeMap::new();
        mp.insert(0, 1);
        dfs(&root, &mut mp, 0, target_sum)
    }
}

fn main() {
    // assert_eq!(
    //     Solution::find_number_of_lis(vec![1, 2, 4, 3, 5, 4, 7, 2]),
    //     3
    // );
    // assert_eq!(Solution::find_number_of_lis(vec![1, 3, 5, 4, 7]), 2);
    // assert_eq!(Solution::find_number_of_lis(vec![2, 2, 2, 2, 2]), 5);
}
