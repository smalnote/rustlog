pub struct Solution;

// 给定一个升序数组,其中间 k 处发知反转,变成
// [k, k+1, ..., n-1, 0, 1, .. k-1]
// 其中搜索 target
// 先找出反转后的断点,然后在两个分段中搜索
//
// [k, k+1, ..., n-1, 0, k, .. k-1]
// lo = 0, hi = n-1, mid = (lo+hi) / 2
// nums[mid] > nums[hi] 则说明断点在 mid ~ hi
// nums[mid] < nums[lo] 则说明断点在 lo ~ mid-1
// 否则 nums[mid] <= nums[hi] && nums[mid] >= nums[lo]
// 这种情况下没法判断,只能向mid 两边扩展,直到 nums[mid+k] != nums[mid]
// 向左 nums[mid-k] > nums[mid-k+1] 则断点是 mid-k [0~k][k+1...n-1]
// 向右 nums[mid+k] < nums[mid+k-1] 则断点是 mid+k-1 [0~k-1][k...n-1]
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let k = Self::find_break_point(&nums);
        (target >= *nums.first().unwrap() && nums[0..k + 1].binary_search(&target).is_ok())
            || (target <= *nums.last().unwrap() && nums[k + 1..].binary_search(&target).is_ok())
    }
    fn find_break_point(nums: &[i32]) -> usize {
        let (mut lo, mut hi) = (0, nums.len() - 1);
        while lo < hi {
            let mid = (lo + hi) / 2;
            if nums[mid] > nums[hi] {
                if lo == mid {
                    return mid;
                }
                lo = mid;
            } else if nums[mid] < nums[lo] {
                if hi == mid {
                    return mid;
                }
                hi = mid;
            } else {
                for k in 1.. {
                    if k <= mid && mid - k >= lo && nums[mid - k] > nums[mid - k + 1] {
                        return mid - k;
                    }

                    if mid + k <= hi && nums[mid + k] < nums[mid + k - 1] {
                        return mid + k - 1;
                    }

                    // k out of range [lo, hi]
                    // [a, a, a, a, a] -> [a, a, a, a, a]
                    if (mid < k || mid - k < lo) && mid + k > hi {
                        return nums.len() - 1;
                    }
                }
            }
        }
        lo
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn test_example() {
        assert!(Solution::search(
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1],
            2
        ));
        assert!(!Solution::search(vec![1], 0));
        assert!(!Solution::search(vec![1, 1], 0));
        assert!(Solution::search(vec![1, 1], 1));
        assert!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 0));
        assert!(!Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 3));
        assert!(!Solution::search(vec![2, 2, 2, 2, 2, 2, 1, 2, 2], 3));
    }
}
