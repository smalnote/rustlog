use std::collections::HashSet;

pub struct Solution {}
/*
 * @lc app=leetcode id=46 lang=rust
 *
 * [46] Permutations
 */

// @lc code=start
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut track = Vec::with_capacity(nums.len());
        let mut used = Default::default();
        let mut result = Default::default();

        permute(&nums, &mut used, &mut track, &mut result);
        result
    }
}

fn permute(
    nums: &Vec<i32>,
    used: &mut HashSet<i32>,
    track: &mut Vec<i32>,
    results: &mut Vec<Vec<i32>>,
) {
    if track.len() == nums.len() {
        let result = track.clone();
        results.push(result);
        return;
    }

    for num in nums {
        if !used.contains(num) {
            used.insert(*num);
            track.push(*num);
            permute(nums, used, track, results);
            track.pop();
            used.remove(num);
        }
    }
}
// @lc code=end

mod tests {

    #[test]
    fn test_1() {
        let nums = vec![1, 2, 3];
        let result = super::Solution::permute(nums);
        assert_eq!(
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1],
            ],
            result
        );
    }
}
