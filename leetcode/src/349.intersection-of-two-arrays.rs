pub struct Solution;

// 求两个数组的交集,交集需要去重.
// 两个数组都排序,遍历短的,跳过重复的,去搜索长的里面有没有.
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let (mut nums1, mut nums2) = if nums1.len() < nums2.len() {
            (nums1.clone(), nums2.clone())
        } else {
            (nums2.clone(), nums1.clone())
        };
        nums1.sort();
        nums2.sort();

        let mut intersection = Vec::new();
        let mut prev = -1;
        for n in nums1 {
            if n != prev && nums2.binary_search(&n).is_ok() {
                intersection.push(n);
            }
            prev = n;
        }
        intersection
    }
}
