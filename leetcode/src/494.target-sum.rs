#[derive(Debug)]
pub struct Solution {}
// @leet start
impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let mut summer = Summer {
            nums,
            target,
            count: 0,
        };
        summer.backtrack(0, 0);
        summer.count
    }
}

struct Summer {
    nums: Vec<i32>,
    target: i32,
    count: i32,
}

impl Summer {
    fn backtrack(&mut self, start: usize, mut sum: i32) {
        if start == self.nums.len() {
            if sum == self.target {
                self.count += 1;
            }
            return;
        }
        sum += self.nums[start];
        self.backtrack(start + 1, sum);
        sum -= self.nums[start];
        sum -= self.nums[start];
        self.backtrack(start + 1, sum);
    }
}
// @leet end
