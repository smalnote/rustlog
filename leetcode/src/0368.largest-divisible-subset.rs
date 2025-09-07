pub struct Solution;

/// 给定一个整数数组,找出其一个最大的子集,这个子集中的任意两个数 a[i], a[j]
/// 要求满足 a[i] % a[j] == 0 或 a[j] % a[i] == 0
///
/// 将子集按升序排列, a[0] a[1] a[2] .... a[n]
/// 则有 a[i] % a[i-1] == 0
/// 若要把一个数加到其中,则需要 new % a[n] == 0, 因为 a[n] 是其前面所有数的倍数了,
/// 所以满足 new  % a[0~n-1] == 0
/// 将数组按升序排序,记录以 nums[i] 为结尾的子集的大小 dp[i]
/// dp[i] = max(dp[i-k] + 1 if nums[i] % nums[i-k] == 0)
/// 题目求的是子集,需要记录 nums[i] 前一个数的下标
impl Solution {
    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums.clone();
        nums.sort();
        let mut dp = vec![(0, 0); nums.len()];
        let mut max = (1, 0); // 记录当前最大子集大小以及结束位置
        for (i, num) in nums.iter().enumerate() {
            // 以 nums[i] 结尾,长度为 1,前一个数下标为自身,表示没有前一个
            dp[i] = (1, i);
            // num 最大的因数除了自身就是 num/2
            // 在 nums 0..i-1 中找第一个>= num/2 的下标,从这个下标开始往回搜索
            // 注意,当 == num/2 时,下标加1,因为后面用 r 时 (0..r) 不包含 r
            let r = (nums[..i])
                .binary_search(&(num / 2))
                .map_or_else(|i| i, |i| i + 1);
            for j in (0..r).rev() {
                if *num % nums[j] == 0 && dp[i].0 < dp[j].0 + 1 {
                    dp[i] = (dp[j].0 + 1, j)
                }
                // 以 nums[j] 结尾最多为 j+1 个, 能构成最多 j+2 个; 不能超过当前最大,没必要往小了找
                if j + 2 < dp[i].0 {
                    break;
                }
            }
            if max.0 < dp[i].0 {
                max.0 = dp[i].0;
                max.1 = i;
            }
        }
        let mut answer = vec![0; max.0];
        let mut j = max.1;
        for i in (0..max.0).rev() {
            answer[i] = nums[j];
            j = dp[j].1;
        }
        answer
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn test_example() {
        assert_eq!(
            Solution::largest_divisible_subset(vec![1, 3, 2]),
            vec![1, 2]
        );
        assert_eq!(
            Solution::largest_divisible_subset(vec![1, 2, 4, 6, 3, 8]),
            vec![1, 2, 4, 8]
        );
    }
}
