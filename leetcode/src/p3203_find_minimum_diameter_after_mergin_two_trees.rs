/// Leetcode 3203. Find Minimum Diameter After Merging Two Trees
/// 给定两棵树，每个节点由一个整数标识，定义树的直径为所有任意两个节点的路径长度
/// 中最长的，将两棵树的任意两个节点连接，求合并后的树可能的最小直径。
///
/// # Arguments
///
/// * `edges1` - 第一棵树的边，每个边由边的两个顶点标识
/// * `edges2` - 第二棵树的边，每个边由边的两个顶点标识
///
/// # Returns
/// 求合并后的树可能的最小直径
///
/// 思路：分别求两棵树的直径, d1, d2，要使合并后的直径，则应该将连接点设置在
/// d1 和 d2 的中间节点，分三种情况:
/// 合并后直径在 tree1 中, result = d1
/// 合并后直径在 tree2 中, result = d2
/// 合并后直径在两棵树中各有一部分, d12 = (d1+1)/2 + (d2+1)/2 + 1
/// 则取 max(d1, d2, d12)
/// 如何求一棵树的直径，可以用 DFS 或 BFS，遍历所有节点，计算经过该节点的最长路
/// 径，所最长的，即为树的直径；经过一个节点的最长路径分两种情况，一种是节点是
/// 端点，则最长路径为节点可以到达的最深节点，另一种是节点是中间点，则最长路径为
/// 以节点为起点的两个最深路径的和。
///
/// Accepted: runtime 50%, memory 66.7%
pub struct Solution();
// @leet start
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn minimum_diameter_after_merge(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> i32 {
        let d1 = Solution::diameter(edges1);
        let d2 = Solution::diameter(edges2);
        let d12 = (d1 + 1) / 2 + (d2 + 1) / 2 + 1;
        let result = if d1 > d2 { d1 } else { d2 };
        if result > d12 {
            result
        } else {
            d12
        }
    }

    fn diameter(edges: Vec<Vec<i32>>) -> i32 {
        let mut tree = Tree::new(edges);
        tree.diameter()
    }
}

struct Tree {
    // nodes' siblings
    nexts: HashMap<i32, Vec<i32>>,
    diameter: i32,
    visited: HashSet<i32>,
}

impl Tree {
    fn new(edges: Vec<Vec<i32>>) -> Tree {
        let mut nexts = HashMap::with_capacity(edges.len() + 1);
        for edge in edges {
            nexts.entry(edge[0]).or_insert_with(Vec::new).push(edge[1]);
            nexts.entry(edge[1]).or_insert_with(Vec::new).push(edge[0]);
        }
        Tree {
            visited: HashSet::with_capacity(nexts.len()),
            nexts,
            diameter: 0,
        }
    }

    fn diameter(&mut self) -> i32 {
        if let Some(first) = self.nexts.iter().next() {
            self.dfs(*first.0);
            self.diameter
        } else {
            0
        }
    }

    // calculate the deepest length from start to unvisited nodes
    fn dfs(&mut self, start: i32) -> i32 {
        // p, q donates top 2 deepest length from start to unvisited nodes, p > q
        let (mut p, mut q) = (0, 0);
        self.visited.insert(start);
        for next in self.nexts.remove(&start).unwrap_or_default() {
            if !self.visited.contains(&next) {
                match self.dfs(next) + 1 {
                    v if v > p => {
                        q = p;
                        p = v;
                    }
                    v if v > q => {
                        q = v;
                    }
                    _ => {}
                }
            }
        }
        let current_diameter = p + q;
        if self.diameter < current_diameter {
            self.diameter = current_diameter
        }

        p
    }
}
// @leet end

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_case_0() {
        let result = Solution::minimum_diameter_after_merge(vec![], vec![]);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_case_1() {
        let result = Solution::minimum_diameter_after_merge(
            vec![vec![0, 1], vec![0, 2], vec![0, 3]],
            vec![vec![0, 1]],
        );
        assert_eq!(result, 3);
    }

    #[test]
    fn test_case_2() {
        let result = Solution::minimum_diameter_after_merge(
            vec![
                vec![0, 1],
                vec![0, 2],
                vec![0, 3],
                vec![2, 4],
                vec![2, 5],
                vec![3, 6],
                vec![2, 7],
            ],
            vec![
                vec![0, 1],
                vec![0, 2],
                vec![0, 3],
                vec![2, 4],
                vec![2, 5],
                vec![3, 6],
                vec![2, 7],
            ],
        );
        assert_eq!(result, 5);
    }
}
