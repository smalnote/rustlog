use std::collections::HashSet;

pub struct Solution;

// 给定一个有向图，用二维数组表示 graph[i], 每个 graph[i] 是节点 i 的下一个节点列表，
// 如果一个节点没有一下个节点，则这个节点是最终节点，如果一个节点作为起点的所有可能的
// 路径是都是最终节点，则这个是安全节点，按顺序返回所有安全节点。
//
// 采用 DFS + MEM + Backtrack
impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let (mut safe, mut not_safe) = (HashSet::new(), HashSet::new());
        let mut terminals = Vec::new();
        for start in 0..graph.len() {
            let mut chain = HashSet::new();
            chain.insert(start as i32);
            if Self::dfs(&graph, start, &mut safe, &mut not_safe, &mut chain) {
                terminals.push(start as i32);
            }
        }
        terminals
    }

    fn dfs(
        graph: &Vec<Vec<i32>>,
        start: usize,
        safe: &mut HashSet<usize>,
        not_safe: &mut HashSet<usize>,
        chain: &mut HashSet<i32>,
    ) -> bool {
        if graph[start].is_empty() {
            safe.insert(start);
            true
        } else if safe.contains(&start) {
            true
        } else if not_safe.contains(&start) {
            false
        } else {
            for &next in graph[start].iter() {
                if chain.contains(&next) {
                    return false;
                }
                chain.insert(next);
                if !Self::dfs(graph, next as usize, safe, not_safe, chain) {
                    not_safe.insert(start);
                    chain.remove(&next);
                    return false;
                }
                chain.remove(&next);
            }
            safe.insert(start);
            true
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn test_example() {
        assert_eq!(
            Solution::eventual_safe_nodes(
                vec![vec![], vec![0, 2, 3, 4], vec![3], vec![4], vec![],]
            ),
            vec![0, 1, 2, 3, 4]
        );
        assert_eq!(
            Solution::eventual_safe_nodes(vec![
                vec![1, 2, 3, 4],
                vec![1, 2],
                vec![3, 4],
                vec![0, 4],
                vec![]
            ]),
            vec![4]
        );
        assert_eq!(
            Solution::eventual_safe_nodes(vec![
                vec![1, 2],
                vec![2, 3],
                vec![5],
                vec![0],
                vec![5],
                vec![],
                vec![]
            ]),
            vec![2, 4, 5, 6]
        );
    }
}
