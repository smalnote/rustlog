pub struct Solution;

// 求两个数组的交集,对于复元素,取两个数组中数量较少的作为最终数量
// 两个数组都排序,遍历短的,跳过重复的,去搜索长的里面有没有.
impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let (mut nums1, mut nums2) = if nums1.len() < nums2.len() {
            (nums1.clone(), nums2.clone())
        } else {
            (nums2.clone(), nums1.clone())
        };
        nums1.sort();
        nums2.sort();

        let mut intersection = Vec::new();
        let mut prev = -1;
        let mut count = 0;
        for n in nums1 {
            if n != prev {
                count = nums2.partition_point(|&x| x <= n) - nums2.partition_point(|&x| x < n);
            }
            if count > 0 {
                intersection.push(n);
                count -= 1;
            }
            prev = n;
        }
        intersection
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn test_example() {
        assert_eq!(
            Solution::intersect(vec![1, 2, 1, 1, 3, 9], vec![9, 1, 1, 2, 2]),
            vec![1, 1, 2, 9]
        );
    }
}
