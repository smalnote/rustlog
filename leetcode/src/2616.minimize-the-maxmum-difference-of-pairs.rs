pub struct Solution;
/// 从 nums 中找取 p 对数，p 对数定义的差值为其中最大的一对数的差值
/// 求取 p 对数，使其最大差值在所有可能的 p 对数中最小。
///
/// 采用二分查找，先定好 r，求满足 diff <= r 的 paris 数 pr
/// 如果 pr < p, 增大 r , pr > p，减少 r
/// lo = 0, hi = max(nums) - min(nums)
impl Solution {
    pub fn minimize_max(nums: Vec<i32>, p: i32) -> i32 {
        let mut nums = nums.clone();
        nums.sort();
        let (mut lo, mut hi) = (0, nums[nums.len() - 1] - nums[0]);

        while lo < hi {
            let mid = (lo + hi) / 2;
            let mut pr = 0;
            let mut i = 0;
            while i + 1 < nums.len() {
                if nums[i + 1] - nums[i] <= mid {
                    pr += 1;
                    i += 2;
                } else {
                    i += 1;
                }
            }
            if pr >= p {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example_1() {
        assert_eq!(1, Solution::minimize_max(vec![10, 1, 2, 7, 1, 3], 2));
    }
}
