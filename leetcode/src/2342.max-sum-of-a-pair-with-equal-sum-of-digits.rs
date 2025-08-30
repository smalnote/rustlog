pub struct Solution;

// 找出数组中一对数，其每个位上的数字和相同，且这对数的和最大，返回这个和
use std::collections::HashMap;
impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        let mut m = HashMap::new();
        let mut result = -1;
        for &num in nums.iter() {
            let mut n = num;
            let mut digit_sum = 0;
            while n > 0 {
                digit_sum += n % 10;
                n /= 10;
            }
            m.entry(digit_sum)
                .and_modify(|old| {
                    result = std::cmp::max(result, *old + num);
                    *old = std::cmp::max(*old, num);
                })
                .or_insert(num);
        }
        result
    }
}
