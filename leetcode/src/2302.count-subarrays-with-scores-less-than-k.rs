pub struct Solution;

// 子数组的 score = 子数组的和 * 子数组的长度
// 求 score < k 子数组的长度
// 以 start 为起点，对 k 进行二分查找，找到 score >= k 的 [start..=end]
// 则有 end - start + 1 个，复杂度 n * log(n)
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
        let mut prefix_sums = Vec::with_capacity(nums.len());
        let mut sum = 0;
        for (index, &num) in nums.iter().enumerate() {
            sum += num as i64;
            prefix_sums.push((index, sum));
        }

        let mut count = 0;

        for start in 0..prefix_sums.len() {
            let pre_sum = if start > 0 {
                prefix_sums[start - 1].1
            } else {
                0
            };
            let slice = &prefix_sums[start..];
            match slice.binary_search_by(|&(end, sum)| {
                ((sum - pre_sum) * (end - start + 1) as i64).cmp(&k)
            }) {
                Ok(index) => {
                    count += index;
                }
                Err(index) => {
                    count += index;
                }
            }
        }

        count as i64
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn test_examples() {
        assert_eq!(Solution::count_subarrays(vec![2, 1, 4, 3, 4], 10), 6);
        assert_eq!(Solution::count_subarrays(vec![1, 1, 1], 5), 5);
    }
}
