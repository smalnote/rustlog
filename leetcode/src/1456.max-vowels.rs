/*
 * @lc app=leetcode id=1456 lang=rust
 * topic="sliding window"
 * [1456] Maximum number of vowels in a substring of given length
 */

// @lc code=start
impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let k = k as usize;
        let mut max = 0;
        let mut current = 0;
        let chars: Vec<char> = s.chars().collect();
        for c in &chars[0..k] {
            if matches!(c, 'a' | 'e' | 'i' | 'o' | 'u') {
                current += 1;
                max = current;
            }
        }
        for i in k..chars.len() {
            if matches!(chars[i - k], 'a' | 'e' | 'i' | 'o' | 'u') {
                current -= 1;
            }
            if matches!(chars[i], 'a' | 'e' | 'i' | 'o' | 'u') {
                current += 1;
            }
            if max < current {
                max = current
            }
        }

        max
    }
}
// @lc code=end
pub struct Solution {}

mod tests {
    #[test]
    fn test_try_hard() {
        let max = super::Solution::max_vowels("tryhard".into(), 4);
        assert_eq!(max, 1);
    }
}
