pub struct Solution;
/// 给定两个数组，求从两个数组中各取出一个数进行 xor 的得到所有可可能的组合后 xor 的数，
/// 这引起数进行 xor，返回得到的最后的数，如 nums1.length=2, nums2.length=3
/// 则有 2 * 3 共 6 对，6对 xor 后得到 6 个数，再进行 xor，返回其值
///
/// nums1 中的数会 xor nums1.length 次
/// nums2 中的数会 xor nums2.length 次
/// 一个数 xor 偶数次为 0，奇数次为自身
impl Solution {
    pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut r = 0;
        let (l1, l2) = (nums1.len(), nums2.len());
        if l1 & 1 == 1 {
            r = nums2.iter().fold(r, |acc, x| acc ^ x);
        }
        if l2 & 1 == 1 {
            r = nums1.iter().fold(r, |acc, x| acc ^ x);
        }
        r
    }
}
