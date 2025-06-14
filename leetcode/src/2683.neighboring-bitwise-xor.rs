pub struct Solution;

/// 给定数组 nums
/// derived[i] = nums[i] xor nums[(i+1)^nums.len()]
/// 已知 derived, 求是否存在 nums
/// nums 中每个数都 xor 两遍得到 derived
/// 因此 derived 中所有的数 xor 后应该为 0
impl Solution {
    pub fn does_valid_array_exist(derived: Vec<i32>) -> bool {
        derived.iter().fold(0, |a, c| a ^ c) == 0
    }
}
