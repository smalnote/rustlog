use std::cmp::min;

pub struct Solution;

// 给定一个正整数数组 nums 和正整数 k, 最多可以将数组分成 k 份，每一份的得分是
// 该组的平均数，求可能获得的最大总行分
// 记 dp[i][j] 为前 i 个数 [0,i-1] 分成 j 份的最大得分，j <= i（i个数最多能分 i 份）
// dp[i][1] = prefix_sums[i] / i
// dp[i][j] = max(dp[p][j-1] + avg(p, i)) for j-1 <= p <= i-1
// prefix_sums[i] = sum(nums[0]~nums[i-1])
// avg(p, i) = (prefix_sums[i] - prefix_sums[p]) / (i - p)
impl Solution {
    pub fn largest_sum_of_averages(nums: Vec<i32>, k: i32) -> f64 {
        let mut dp = vec![vec![0 as f64; k as usize + 1]; nums.len() + 1];

        let mut prefix_sums = vec![0; nums.len() + 1];
        let mut prev = 0;
        for (i, &num) in nums.iter().enumerate() {
            prev += num;
            prefix_sums[i + 1] = prev;
        }

        for i in 1..=nums.len() {
            dp[i][1] = prefix_sums[i] as f64 / i as f64;
            for j in 2..=min(i, k as usize) {
                let mut max = 0 as f64;
                for p in (j - 1)..=(i - 1) {
                    let curr = dp[p][j - 1]
                        + ((prefix_sums[i] - prefix_sums[p]) as f64) / ((i - p) as f64);
                    if max < curr {
                        max = curr;
                    }
                }
                dp[i][j] = max;
            }
        }
        dp[nums.len()][k as usize]
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn test_example() {
        assert_eq!(
            Solution::largest_sum_of_averages(vec![1, 2, 3, 4, 5, 6, 7], 4),
            20.5
        );
        assert_eq!(
            Solution::largest_sum_of_averages(vec![9, 1, 2, 3, 9], 3),
            20.0
        );
    }
}
