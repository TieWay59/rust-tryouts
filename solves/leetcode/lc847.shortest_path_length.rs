struct Solution {}

impl Solution {
    pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
        let mut q = std::collections::VecDeque::with_capacity(1 << graph.len());
        let n = graph.len() as u32;
        for i in 0..n {
            q.push_back((i, 1i32 << i, 0)); // node,  state
        }

        let mut ret = i32::MAX;
        while let Some((u, s, l)) = q.pop_front() {
            if s.count_ones() == n {
                ret = ret.min(l);
                break;
            }
            for &v in &graph[u as usize] {
                if (1 << v) & s == 0 {
                    q.push_front((v as u32, (1 << v) | s, l + 1));
                } else {
                    q.push_back((v as u32, (1 << v) | s, l + 1));
                }
            }
        }
        ret
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::shortest_path_length(vec![vec![1, 2, 3], vec![0], vec![0], vec![0]])
    );
    println!(
        "{:?}",
        Solution::shortest_path_length(vec![
            vec![1],
            vec![0, 2, 4],
            vec![1, 3, 4],
            vec![2],
            vec![1, 2]
        ])
    );
}
