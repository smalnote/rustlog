pub struct Solution;

use std::{cmp::max, collections::HashMap};

impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let mut max_freq = 0;
        let mut counts: HashMap<i32, i32> = Default::default();
        nums.into_iter().for_each(|num| {
            max_freq = max(
                max_freq,
                *(counts
                    .entry(num)
                    .and_modify(|count| *count += 1)
                    .or_insert(1)),
            );
        });
        let max_count = counts.into_iter().filter(|(_, c)| *c == max_freq).count();
        max_count as i32 * max_freq
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_example() {
        assert_eq!(
            super::Solution::max_frequency_elements(vec![1, 2, 2, 3, 1, 4]),
            4
        );
        assert_eq!(
            super::Solution::max_frequency_elements(vec![1, 2, 3, 4, 5]),
            5
        );
    }
}
