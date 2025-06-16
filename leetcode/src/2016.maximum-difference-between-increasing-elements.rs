pub struct Solution;

// 求 nums[j] - nums[i] >= 0, j > i 的最大值
impl Solution {
    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        let mut max = -1;
        let mut curr = nums[0];
        for num in &nums[1..] {
            if curr < *num {
                if max < *num - curr {
                    max = *num - curr;
                }
            } else {
                curr = *num
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_one() {
        assert_eq!(
            Solution::maximum_difference(vec![9, 8, 7, 6, 5, 4, 3, 2, 1]),
            -1
        );
    }
}
