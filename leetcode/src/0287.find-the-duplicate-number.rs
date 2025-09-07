pub struct Solution;

// 给定数组长度为 n+1, 只包含 [1,n] 范围内的数,有且仅有一个数在数组内重复出现,找
// 出这个数, 不能改为数组,只能使用额外的固定空间.
//
// 二分查找法: 取 mid, [lo,mid] 最多有 mid - lo + 1 个不同的数, 计算数组中在范围 [lo, mid] 的个数 c,
// 若 c > mid - lo + 1 则重复的数在 [lo,mid) 范围内,否则在 [mid, hi)
impl Solution {
    pub fn find_dup(nums: Vec<i32>) -> i32 {
        let (mut lo, mut hi) = (1, nums.len() as i32 - 1);
        while lo < hi {
            let mid = (lo + hi) / 2;
            let mut count = 0;
            for num in nums.iter() {
                if lo <= *num && *num <= mid {
                    count += 1;
                }
            }
            if count > (mid - lo + 1) {
                hi = mid;
            } else {
                lo = mid + 1;
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
        assert_eq!(Solution::find_dup(vec![1, 3, 4, 2, 2]), 2);
        assert_eq!(Solution::find_dup(vec![3, 1, 3, 4, 2]), 3);
        assert_eq!(Solution::find_dup(vec![3, 3, 3, 3, 3]), 3);
    }
}
