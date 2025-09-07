pub struct Solution;

// 在一个递增的数组中找出 i, j(i<j) 有 nums[i] + nums[j] == target
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..numbers.len() {
            if i > 0 && numbers[i] == numbers[i - 1] {
                continue;
            }
            if let Ok(j) = &numbers[i + 1..].binary_search(&(target - numbers[i])) {
                return vec![(i + 1) as i32, (i + 1 + *j + 1) as i32];
            }
        }
        panic!("not found")
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn test_example() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![2, 3, 4], 6), vec![1, 3]);
    }
}
