pub struct Solution;

// 给定一个严格递增数组,其被旋转一次时,所有数的下标 +1, 超出则回环到开始
// [1, 2, 3, 4] 旋转2次-> [3, 4, 1, 2]
//
// 二分查找, 当 nums[mid] < nums[lo], 分割点在 lo ~ mid
// 当 nums[mid] > nums[hi], 分割点在 mid ~ hi
// 分割点 k 有 nums[k] > nums[k+1], 则 nums[k+1] 是开始位置也就是最小的
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        if nums.first().unwrap() <= nums.last().unwrap() {
            return *nums.first().unwrap();
        }
        let (mut lo, mut hi) = (0, nums.len() - 1);
        while hi > 0 && lo < hi - 1 {
            let mid = (lo + hi) / 2;
            if nums[mid] < nums[lo] {
                hi = mid;
            } else if nums[mid] > nums[hi] {
                lo = mid;
            } else {
                panic!("invalid nums");
            }
        }
        nums[lo + 1]
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn test_example() {
        assert_eq!(Solution::find_min(vec![3, 4, 5, 1, 2]), 1);
        assert_eq!(Solution::find_min(vec![3, 4, 5]), 3);
    }
}
