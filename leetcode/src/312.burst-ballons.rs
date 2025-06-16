pub struct Solution;

// 给定 nums
// 每次从 nums 中删除一个数 nums[i] ,获得 nums[i-1] * nums[i] * nums[i+1]
// 当 i-1, i+1 超出边界时,其值为1
// 求删除完所有的数能得到的最大值
//
// 记 dp[i][j] nums[i~j] 的解
// 则 dp[i][j] = max(nums[k]*nums[i-1]*nums[j+1] + dp[i][k-1] + dp[k+1][j]) i <= k <= j
// nums[k] 是 nums[i~j] 中最后一个删除的,则删除 nums[k] 时,其左边为 nums[i-1] 右边为 nums[j+1]
// dp[i][j] 依赖于 dp[i][<=j] 和 d[>=i][j], i >= j
// 从最后一行往上,再从左到右构造 dp
impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = Vec::with_capacity(n);
        for _ in 0..n {
            dp.push(vec![0; n]);
        }
        for i in (0..n).rev() {
            for j in i..n {
                let mut max = 0;
                for k in i..=j {
                    let t = nums[k]
                        * if i > 0 { nums[i - 1] } else { 1 }
                        * if j < n - 1 { nums[j + 1] } else { 1 }
                        + if k > i { dp[i][k - 1] } else { 0 }
                        + if k < j { dp[k + 1][j] } else { 0 };
                    if max < t {
                        max = t;
                    }
                }
                dp[i][j] = max;
            }
        }
        dp[0][n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn test_max_coins() {
        assert_eq!(Solution::max_coins(vec![3, 1, 5, 8]), 167);
        assert_eq!(Solution::max_coins(vec![1, 5]), 10);
    }
}
