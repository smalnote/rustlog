pub struct Solution;

/// 给定一个数组,找出其中一个 peak, nums[peak] > nums[peak-1] && nums[peak] > nums[peak+1]
/// 对于任意 i 有 nums[i] != nums[i+1]
///
/// 二分搜索 lo, hi, mid
/// 检查 nums[mid]
/// 若 nums[mid] 是 peak 直接返回
/// 若 nums[mid] < nums[mid-1] 且 nums[lo] < nums[mid-1] 则 lo ~ mid-1 必有 peak
/// 若 nums[mid] < nums[mid+1] 且 nums[hi] < nums[mid+1] 则 mid+1 ~ hi 必有 peak
/// 否则可能两边都有
impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        Self::find_range_peak(&nums, 0, nums.len() - 1)
            .map(|v| v as i32)
            .unwrap()
    }

    fn find_range_peak(nums: &[i32], lo: usize, hi: usize) -> Option<usize> {
        let mid = (lo + hi) / 2;
        if (mid == 0 || nums[mid] > nums[mid - 1])
            && (mid + 1 == nums.len() || nums[mid] > nums[mid + 1])
        {
            Some(mid)
        } else if mid > 0 && nums[mid] < nums[mid - 1] && nums[lo] < nums[mid - 1] {
            Self::find_range_peak(nums, lo, mid)
        } else if mid < hi && nums[mid] < nums[mid + 1] && nums[hi] < nums[mid + 1] {
            Self::find_range_peak(nums, mid, hi)
        } else {
            if lo < mid {
                let left = Self::find_range_peak(nums, lo, mid);
                if left.is_some() {
                    return left;
                }
            }
            if mid < hi {
                Self::find_range_peak(nums, mid + 1, hi)
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_example() {
        assert_eq!(Solution::find_peak_element(vec![1, 2]), 1);
        assert_eq!(Solution::find_peak_element(vec![1]), 0);
        assert_eq!(Solution::find_peak_element(vec![1, 2, 3, 1]), 2);
        assert_eq!(Solution::find_peak_element(vec![1, 2, 1, 3, 5, 6, 4]), 5);
    }
}
