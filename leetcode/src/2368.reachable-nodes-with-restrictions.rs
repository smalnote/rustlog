pub struct Solution;

use std::collections::{HashMap, HashSet, VecDeque};
// 给定 n 个节点，由 n-1 条边连接成一个无向无环的树，还有一个限制访问的节点列表
// 求可以通过节点 0 (不在限制列表中) 访问的节点数量
impl Solution {
    pub fn reachable_nodes(_: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> i32 {
        let restricted: HashSet<_> = restricted.iter().collect();
        let mut nexts = HashMap::new();
        let mut tmp = edges.iter().map(Vec::as_slice);
        while let Some([a, b, ..]) = tmp.next() {
            nexts
                .entry(a)
                .and_modify(|list: &mut Vec<i32>| list.push(*b))
                .or_insert(vec![*b]);
            nexts
                .entry(b)
                .and_modify(|list| list.push(*a))
                .or_insert(vec![*a]);
        }

        let mut count = 0;
        let mut q = VecDeque::new();
        let mut visited = HashSet::new();
        q.push_back(0);
        while let Some(c) = q.pop_front() {
            count += 1;
            visited.insert(c);
            if let Some(adjcents) = nexts.get(&c) {
                for &next in adjcents.iter() {
                    if !visited.contains(&next) && !restricted.contains(&next) {
                        visited.insert(next);
                        q.push_back(next);
                    }
                }
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn test_example() {
        assert_eq!(
            Solution::reachable_nodes(
                7,
                vec![
                    vec![0, 1],
                    vec![1, 2],
                    vec![3, 1],
                    vec![4, 0],
                    vec![0, 5],
                    vec![5, 6]
                ],
                vec![4, 5]
            ),
            4
        );
    }
}
