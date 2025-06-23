pub struct Solution;

use std::collections::HashMap;

// 检查是否可以把一个数组分成平均数相同的两部分
// 记数组长度为 n, 总和为 total, 若可以分成两部分，平均数相等，记第一部分长度为 k，和为 s
// 则有两部分平均数等式 s / k = (total - s) / (n-k)
// 即 s * (n-k) = (total - s) * k
// 问题转换为求是否可以从数组中挑出 k 个数，其和为 s 满足等式
// 记 dp[i][k][s] 为前 i 个数是否中的 k 个是否可以求和得到 s
// 由于 s 可能非常大，把其换作 map
// dp[i][k]{s}, k < n 由于 k  的对称性，k <= (n+1)/2
// dp[i][k]{s} = dp[i-1][k-1][s-nums[i]]
// dp[i] 只依赖于 dp[i-1] 可以省略一个维度
// 迭代 dp[k]{s}
// 数组一部分的总和 <= total, 即 s <= total (等于的情况是全部为0)
// 初始化 0 个和为 0 dp[0]{0}
// 对于每个数， 都有加和不加两种选择
// 不加时， {s} 没变化，所以直接把 nums[i] + 已有的 s 放进集合即可
// 注意迭代的时间 k 从大到小，避免 dp[k-1]{prev+nums[i]}, dp[k]{prev+nums[i]+nums[i]}
// 一个数重复使用
impl Solution {
    pub fn split_array_same_average(nums: Vec<i32>) -> bool {
        if nums.len() < 2 {
            return false;
        }
        let total = nums.iter().sum::<i32>();
        let len = nums.len();

        let k = nums.len().div_ceil(2);
        let mut dp = Vec::with_capacity(k + 1);
        for _ in 0..=k {
            dp.push(HashMap::new());
        }
        dp[0] = HashMap::from([(0, true)]);
        for num in nums {
            for j in (1..=k).rev() {
                let (first, second) = dp.split_at_mut(j);
                let dpcurr = &mut second[0];
                for (&sum, _) in first.last().unwrap().iter() {
                    let new_sum = sum + num;
                    if new_sum <= total {
                        dpcurr.insert(new_sum, true);
                        if j as i32 * (total - new_sum) == (len - j) as i32 * new_sum {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn test_example() {
        assert!(Solution::split_array_same_average(vec![0, 0]));
        assert!(!Solution::split_array_same_average(vec![18, 10, 5, 3]));
        assert!(Solution::split_array_same_average(vec![
            1, 2, 3, 4, 5, 6, 7, 8
        ]));
        assert!(!Solution::split_array_same_average(vec![1, 3]));
    }
}
