#[allow(unused)]
struct Solution;

#[allow(unused)]
impl Solution {
    pub fn count_highest_score_nodes(parents: Vec<i32>) -> i32 {
        fn solve_by_dfs(nodes: &Vec<Vec<i32>>, parents_copy: &mut Vec<i64>, i: i32) -> i64 {
            let v = nodes[i as usize].iter().fold((1, 1), |(prod, sum), j| {
                let cnt = solve_by_dfs(&nodes, parents_copy, *j);
                (prod * cnt, sum + cnt)
            });
            parents_copy[i as usize] = v.0 * (1.max(nodes.len() as i64 - v.1));
            v.1
        }

        let (mut nodes, mut parents_copy) =
            (vec![Vec::new(); parents.len()], vec![0; parents.len()]);
        (1..parents.len()).for_each(|i| nodes[parents[i] as usize].push(i as i32));

        solve_by_dfs(&nodes, &mut parents_copy, 0);
        let max_val = parents_copy.iter().max().take().unwrap();
        parents_copy.iter().filter(|v| *v == max_val).count() as i32
    }
}

fn main() {
    println!("Hello, world!");
}
