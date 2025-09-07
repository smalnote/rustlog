pub struct Solution;

// 给定一组信封的宽高,一个信封如果宽和高都大于另一个信封,则可以嵌套,
// 问最多可以连续嵌套多少个
//
// 把信封按 width 升序排列,如果 width 相同, 再按 height 降序排列,
// 问题转化为求在排好序的信封中求按 height 比较的最长递增子序列.
// 为什么需要在 width 相同时按 height 降序排: 相同 width 的信封不能嵌套,
// height 降序排列确保 width 相同时其 height 不会构成递增子序列
//
// LIS 问题可用 dp 解, 构造 dp[i], 对于新加入的 h, 如果 h > dp.last() 则添加,
// 如果 h < dp.last() 则更新到其中比它大的最小那个
impl Solution {
    pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
        let mut envelopes: Vec<(i32, i32)> = envelopes.iter().map(|v| (v[0], v[1])).collect();
        envelopes.sort_by(|&(w1, h1), &(w2, h2)| match w1.cmp(&w2) {
            std::cmp::Ordering::Equal => h2.cmp(&h1),
            other => other,
        });
        let mut dp = Vec::new();
        for env in envelopes {
            match dp.binary_search(&env.1) {
                Ok(_) => {}
                Err(at) => {
                    if at < dp.len() {
                        dp[at] = env.1;
                    } else {
                        dp.push(env.1);
                    }
                }
            }
        }
        dp.len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn test_example() {
        assert_eq!(
            Solution::max_envelopes(vec![vec![5, 4], vec![6, 4], vec![6, 7], vec![2, 3]]),
            3
        );
    }
}
