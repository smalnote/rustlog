pub struct Solution;

use std::collections::{HashMap, HashSet, hash_map::Entry};
// 完全子数组中不同数字的数量与整个数组的数量一致，计算一个数组的完全子数组的数量。
// 双指针法，以 i 开头，找到最小的结尾 j, 使 [i, j] 是完全子数组，则以 i 开头的
// 完全子数组数量为 nums.len() - j, 然后 i + 1, 寻找下一个 j。
impl Solution {
    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        let set: HashSet<i32> = nums.iter().copied().collect();
        let distinct_count = set.len();
        let mut set = HashMap::<i32, usize>::with_capacity(distinct_count);
        let mut count = 0;
        let (mut i, mut j) = (0, 0);
        loop {
            if set.len() < distinct_count {
                if j < nums.len() {
                    set.entry(nums[j]).and_modify(|c| *c += 1).or_insert(1);
                    j += 1;
                } else {
                    break;
                }
            }
            if set.len() == distinct_count {
                count += nums.len() - j + 1;
                {
                    match set.entry(nums[i]) {
                        Entry::Occupied(mut e) => {
                            if *e.get() == 1 {
                                e.remove();
                            } else {
                                *e.get_mut() -= 1;
                            }
                        }
                        Entry::Vacant(_) => {}
                    }
                }
                i += 1;
            }
        }

        count as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn test_examples() {
        assert_eq!(Solution::count_complete_subarrays(vec![1, 3, 1, 2, 2]), 4);
        assert_eq!(Solution::count_complete_subarrays(vec![5, 5, 5, 5]), 10);
    }
}
