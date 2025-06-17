pub struct Solution;

/// 猜数字游戏,给定数字 n, 目标数字在 [1, n] 范围内,每次猜一个数字,如果猜中则游戏
/// 结束,否则付与数字相等的钱,获得该数字是偏大还是偏小的信息;
/// 问确保猜中最少需要准备多少钱.
///
/// 记猜中范围 [i, j] 内的数字的解为 dp[i][j] j >= i
/// dp[i][j] = min(k + max(dp[i][k-1], dp[k+1][j]))
/// 即付 k 后知到范围是 i~k~1 还是 k+1~j, 那只需要取两者中较大的即可
/// 当 i == j 时, dp[i][j] == 0
///
/// dp[i][j] 有效区域占据矩阵的右上三角区, dp[i][j] 依赖左边dp[i][<=j] 和下边 dp[>=i][j]
/// 因此从下往上从左往右构造 dp
impl Solution {
    pub fn get_money_amount(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![vec![0; n + 1]; n + 1];

        for i in (1..=n).rev() {
            for j in i + 1..=n {
                let mut m = i32::MAX;
                for k in i..=j {
                    m = std::cmp::min(
                        m,
                        k as i32
                            + std::cmp::max(dp[i][k - 1], if k + 1 < n { dp[k + 1][j] } else { 0 }),
                    );
                }
                dp[i][j] = m;
            }
        }

        dp[1][n]
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn test_example() {
        assert_eq!(Solution::get_money_amount(10), 16);
        assert_eq!(Solution::get_money_amount(1), 0);
        assert_eq!(Solution::get_money_amount(2), 1);
    }
}
