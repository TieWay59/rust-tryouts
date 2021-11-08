#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution {}
impl Solution {
    pub fn get_kth_from_end(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut slow = &head;
        let mut fast = &head;
        
        for _ in 0..k {
            fast = &fast.as_ref().unwrap().next;
        }
        while fast.is_some() {
            fast = &fast.as_ref().unwrap().next;
            slow = &slow.as_ref().unwrap().next;
        }
        slow.clone()
    }
}
fn main() {
    let l = ListNode::new(1);
    Solution::get_kth_from_end(l.next, 1);
    // println!(
    //     "{:?}",
    //     Solution::get_kth_from_end("1.01".to_string(), "1.001".to_string())
    // );
}
