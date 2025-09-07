/*
 * @lc app=leetcode id=89 lang=rust
 *
 * [89] Gray Code
 */

// @lc code=start
use std::collections::HashSet;
impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let n = 1 << n;
        let mut gray_code = GrayCode {
            used: HashSet::with_capacity(n as usize),
            track: Vec::with_capacity(n as usize),
            n,
        };
        gray_code.used.insert(0);
        gray_code.track.push(0);
        gray_code.backtrack(0);
        gray_code.track
    }
}

impl GrayCode {
    fn backtrack(&mut self, curr: i32) {
        if self.track.len() == self.n as usize {
            return;
        }
        let mut i = 1;
        while i < self.n {
            let next = curr ^ i;
            i <<= 1;
            if self.used.contains(&next) {
                continue;
            }
            self.used.insert(next);
            self.track.push(next);
            self.backtrack(next);
            if self.track.len() == self.n as usize {
                return;
            }
            self.used.remove(&next);
            self.track.pop();
        }
    }
}

struct GrayCode {
    used: HashSet<i32>,
    track: Vec<i32>,
    n: i32,
}
// @lc code=end

pub struct Solution {}

mod tests {

    #[test]
    fn test_gray_code() {
        use super::*;
        dbg!(Solution::gray_code(2));
        dbg!(Solution::gray_code(1));
    }
}
