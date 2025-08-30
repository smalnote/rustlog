pub struct Solution;

// 给定一个长度为 n 的整数数组 vals, vals[i] 代表第 i 个节点的值, 一个 edges 二
// 维数组 edges[i] = [a_i, b_i] 表示节点 a 和 b 之间的一条边.
//
// 一条 Good Path 为长度 >= 1, 两头的节点值相等,中间的节点值 <= 两头的节点的值,
// 注意单个节点也算一条 Good Path, 求有多少条不同的 Good Path.
// 对于相同的值在图中用小于的值连接通起来的集合,有 n 个相同的值,就有 C(n,2)条
// 路径,再加自身 n 条
// 用并查集构造相同值的连通集合
// 为了避免每个值都用一个并查集,从小到大往集合中添加,前面加的小的为后面的重复使用
use std::collections::{BTreeMap, HashMap};

struct DisjointSet {
    parent: Vec<usize>,
    rank: Vec<i32>,
}

impl DisjointSet {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let (xr, yr) = (self.find(x), self.find(y));
        if xr == yr {
            return;
        }
        if self.rank[xr] < self.rank[yr] {
            self.parent[xr] = yr;
        } else if self.rank[xr] > self.rank[yr] {
            self.parent[yr] = xr;
        } else {
            self.parent[yr] = xr;
            self.rank[xr] += 1;
        }
    }
}

impl Solution {
    pub fn number_of_good_paths(vals: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let n = vals.len();
        let mut adj = vec![vec![]; n];
        for edge in edges {
            let (a, b) = (edge[0] as usize, edge[1] as usize);
            adj[a].push(b);
            adj[b].push(a);
        }

        // map from val to nodes with that value
        // 以升序处理值,确保先加进 DSU 的值可以被后加的值作为桥使用
        let mut val_map = BTreeMap::<i32, Vec<usize>>::new();
        for (i, &val) in vals.iter().enumerate() {
            val_map.entry(val).or_default().push(i);
        }

        let mut dsu = DisjointSet::new(n);
        let mut result = 0;

        for (&val, nodes) in val_map.iter() {
            for &node in nodes.iter() {
                for &nei in &adj[node] {
                    if vals[nei] <= val {
                        dsu.union(node, nei);
                    }
                }
            }

            // Count frequency of root per component
            // 计算相同的值在图中形成的连通集,每个集的个数,取组合数
            let mut freq = HashMap::<usize, i32>::new();
            for &node in nodes.iter() {
                let root = dsu.find(node);
                *freq.entry(root).or_insert(0) += 1;
            }

            for &count in freq.values() {
                // For each group: C(n, 2) + n
                result += count * (count + 1) / 2;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_example() {
        assert_eq!(
            Solution::number_of_good_paths(
                vec![2, 5, 5, 1, 5, 2, 3, 5, 1, 5],
                vec![
                    vec![0, 1],
                    vec![2, 1],
                    vec![3, 2],
                    vec![3, 4],
                    vec![3, 5],
                    vec![5, 6],
                    vec![1, 7],
                    vec![8, 4],
                    vec![9, 7]
                ],
            ),
            20
        );
        assert_eq!(
            Solution::number_of_good_paths(
                vec![1, 3, 2, 1, 3],
                vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4]]
            ),
            6
        );
        assert_eq!(
            Solution::number_of_good_paths(
                vec![1, 1, 2, 2, 3],
                vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![2, 4]]
            ),
            7
        );
        assert_eq!(Solution::number_of_good_paths(vec![1], vec![]), 1);
    }
}
