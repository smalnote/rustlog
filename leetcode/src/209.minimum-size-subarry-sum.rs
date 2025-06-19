pub struct Solution;

/// 给定一个只包含正整数的数组 nums, 和正整数 target
/// 求 nums 的最短子数组,其和大于等于 target
///
/// 方法一: 双指针法,线性从左向右扫,如果小了,向右扩展,如果大了,从左收缩
///
/// 方法二: 固定窗口长度搜索,窗口长度最长 nums.len(), 最短 1, 在这个范围内进行
/// 二分查找.
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        Self::binary_search_window_length(target, nums)
    }

    pub fn double_pointer_linear_scan(target: i32, nums: Vec<i32>) -> i32 {
        // [i, j)
        let (mut i, mut j) = (0, 1);
        let mut sum = nums[0];
        if sum >= target {
            return 1;
        }
        let mut min_len = nums.len() + 1;

        while j < nums.len() {
            while sum < target && j < nums.len() {
                sum += nums[j];
                j += 1
            }
            while sum >= target && i < j {
                let len = j - i;
                min_len = min_len.min(len);
                sum -= nums[i];
                i += 1;
            }
        }
        if min_len > nums.len() {
            min_len = 0;
        }
        min_len as i32
    }

    pub fn binary_search_window_length(target: i32, nums: Vec<i32>) -> i32 {
        let (mut lo, mut hi) = (1, nums.len() + 1);
        while lo < hi && lo <= nums.len() {
            // mid 为窗口长度
            let mid = (lo + hi) / 2;

            let mut sum = 0;
            for num in &nums[..mid] {
                sum += *num
            }
            let mut ok = sum >= target;
            if !ok {
                for i in mid..nums.len() {
                    sum -= nums[i - mid];
                    sum += nums[i];
                    if sum >= target {
                        ok = true;
                        break;
                    }
                }
            }

            if ok {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        if lo > nums.len() {
            0
        } else {
            lo as i32
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn test_example() {
        assert_eq!(Solution::min_sub_array_len(9, vec![1, 4, 4]), 3);
        assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
        assert_eq!(Solution::min_sub_array_len(4, vec![1, 4, 4]), 1);
        assert_eq!(
            Solution::min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1]),
            0
        );
    }
}
