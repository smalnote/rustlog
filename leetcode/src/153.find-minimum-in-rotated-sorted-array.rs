pub struct Solution;

// 给定一个严格递增数组,其被旋转一次时,所有数的下标 +1, 超出则回环到开始
// [1, 2, 3, 4] 旋转2次-> [3, 4, 1, 2]
//
// 二分查找, 当 nums[mid] < nums[lo], 分割点在 lo ~ mid
// 当 nums[mid] > nums[hi], 分割点在 mid ~ hi
// 分割点 k 有 nums[k] > nums[k+1], 则 nums[k+1] 是开始位置也就是最小的
// 增加难度: 数组可能有重复
// 相等时从 mid 往两边线性搜索
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let k = Solution::find_break_point(&nums);
        nums[(k + 1) % nums.len()]
    }

    fn find_break_point(nums: &[i32]) -> usize {
        let (mut lo, mut hi) = (0, nums.len() - 1);
        while lo < hi {
            let mid = (lo + hi) / 2;
            if nums[mid] > nums[hi] {
                lo = mid;
            } else if nums[mid] < nums[lo] {
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
            if lo + 1 == hi {
                break;
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
        assert_eq!(Solution::find_min(vec![1, 3]), 1);
        assert_eq!(Solution::find_min(vec![3, 1]), 1);
        assert_eq!(Solution::find_min(vec![3, 4, 5, 1, 2]), 1);
        assert_eq!(Solution::find_min(vec![3, 4, 5]), 3);
        assert_eq!(Solution::find_min(vec![2, 2, 2, 0, 1]), 0);
        assert_eq!(Solution::find_min(vec![2, 2, 2, 0, 1, 2]), 0);
    }
}
