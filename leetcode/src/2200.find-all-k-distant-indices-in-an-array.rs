pub struct Solution;

// 给定一个数组 nums, 整数 k 和 key, 要求列出所有的 i,
// 满足 nums[j] == kye, |i-j| <= k, 即 i k 步之内有等于 key 的数
impl Solution {
    pub fn find_k_distant_indices(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32> {
        let mut result = vec![];
        let mut last = -1;

        for (p, &num) in nums.iter().enumerate() {
            if num == key {
                for i in std::cmp::max(last + 1, p as i32 - k)
                    ..=std::cmp::min(p as i32 + k, nums.len() as i32 - 1)
                {
                    result.push(i);
                }
                last = *result.last().unwrap();
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn test_example() {
        assert_eq!(
            Solution::find_k_distant_indices(vec![2, 2, 2, 2, 2], 2, 2),
            vec![0, 1, 2, 3, 4]
        );
        assert_eq!(
            Solution::find_k_distant_indices(vec![3, 4, 9, 1, 3, 9, 5], 9, 1),
            vec![1, 2, 3, 4, 5, 6]
        );
    }
}
