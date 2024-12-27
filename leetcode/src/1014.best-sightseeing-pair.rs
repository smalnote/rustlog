// 1014. Best Sightseeing Pair
// You are given an integer array values where values[i] represents the value
// of the i^th sightseeing spot. Two sightseeing spots i and j have a distance
// j - i between them.
// The score of a pair (i < j) of sightseeing spots is
// values[i] + values[j] + i - j: the sum of the values of the sightseeing
// spots, minus the distance between them.
// Return the maximum score of a pair of sightseeing spots.
//
// 对于 (index_i, value_p) 对，如果有 (index_j, value_q), j > i, q >= p,
// 则前一个可以安全的丢弃，因为对于 j 往后的(k, r)，有
// p + i < q + j, (j, q) 总是更好的选择
// 即对于 (k, r) 我们只保留 (i<k, p> r) 序列
// 维护一个单调递减栈
pub struct Solution();
// @leet start
impl Solution {
    pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
        let mut mono_stack = vec![];
        let mut max_score = 0;
        let mut iter = values.iter().enumerate().map(|(i, v)| (i as i32, *v));
        let first = iter.next().unwrap();
        mono_stack.push(first);
        for (i, v) in iter {
            loop {
                if mono_stack.is_empty() {
                    break;
                }
                let top = mono_stack[mono_stack.len() - 1];
                let current_score = top.1 + v + top.0 - i;
                if max_score < current_score {
                    max_score = current_score
                }
                if top.1 > v {
                    break;
                }
                mono_stack.pop();
            }
            mono_stack.push((i, v));
        }

        max_score
    }
}
// @leet end

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_135() {
        let score = Solution::max_score_sightseeing_pair(vec![1, 3, 5]);
        assert_eq!(score, 7);
    }
}
