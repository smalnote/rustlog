pub struct Solution;

/// 给定一个数 n,将其分解为 k(>=2) 个正整数的和,使这些数的乘积最大;
///
/// 记 dp[i] 为解
/// dp[i] = max(dp[k] * dp[i-k], i) for 1 <= k < i(lower to <= i/2),
/// 对于 i < n, 如果 i 分解后的乘积小于 i, 则不分解,直接用 i, 如 5 -> 2 * 3, 2 跟 3 分解后的乘积小于自身
/// dp[n] = max(dp[k] * dp[n-k]) for 1 <= k < n(lower to <= n/2)
impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n];
        dp[1] = 1;
        for m in 2..n {
            dp[m] = m;
            for k in 1..=(m / 2) {
                let t = dp[k] * dp[m - k];
                if dp[m] < t {
                    dp[m] = t;
                }
            }
        }
        let mut max = 0;
        for k in 1..=(n / 2) {
            let t = dp[k] * dp[n - k];
            if max < t {
                max = t;
            }
        }
        max as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn test_example() {
        assert_eq!(Solution::integer_break(25), 8748);
        assert_eq!(Solution::integer_break(2), 1);
        assert_eq!(Solution::integer_break(8), 18);
        assert_eq!(Solution::integer_break(10), 36);
        assert_eq!(Solution::integer_break(11), 54);
    }
}
