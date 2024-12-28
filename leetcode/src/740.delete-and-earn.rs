// 740. Delete and Earn
// You are given an integer array nums. You want to maximize the number of
// points you get by performing the following operation any number of times:
// * Pick any nums[i] and delete it to earn nums[i] points. Afterwards, you must
// delete every element equal to nums[i] - 1 and every element equal to nums[i] + 1.
// Return the maximum number of points you can earn by applying the above
// operation some number of times.
//
// 先将 nums 排序，再合并相同数字，用另一个数组记录相同数字的数量
// 有:
// points points[i] 为数字 points[i] < points[i+1]
// counts counst[i] 为数字 points[i] 的个数
// dp_deleted[i] 为 points[0..=i] 且删除 points[i] 的最大点数
// dp_reserved[i] 为 points[0..=i] 且保留 points[i] 的最大点数
// dp_deleted[0] = points[0] * counts[0]
// dp_reserved[0] = 0
// dp_deleted[i] =
//   if points[i] == points[i-1] + 1
//     dp_reserved[i-1] + points[i] * counts[i] :
//   else
//     max(dp_deleted[i-1] + points[i] * counts[i], dp_reserved[i-1] + points[i] * counts[i])
//
// dp_reserved[i] =
//     max(dp_reserved[i-1], dp_deleted[i-1])
// result = max(dp_reserved[n-1], dp_deleted[n-1])

pub struct Solution;
// @leet start
use std::vec::IntoIter;
impl Solution {
    pub fn delete_and_earn(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mut nums = nums.into_iter();
        let mut points = Points {
            current_num: nums.next(),
            nums,
        };
        let (point, count) = points.next().unwrap();
        let (mut dp_deleted, mut dp_reserved) = (point * count, 0);
        let mut last_point = point;
        while let Some((point, count)) = points.next() {
            let (dp_deleted_next, dp_reserved_next);
            if point == last_point + 1 {
                dp_deleted_next = dp_reserved + point * count;
            } else {
                dp_deleted_next = if dp_deleted > dp_reserved {
                    dp_deleted
                } else {
                    dp_reserved
                } + point * count;
            }
            dp_reserved_next = if dp_reserved > dp_deleted {
                dp_reserved
            } else {
                dp_deleted
            };
            (dp_deleted, dp_reserved) = (dp_deleted_next, dp_reserved_next);
            last_point = point;
        }
        if dp_deleted > dp_reserved {
            dp_deleted
        } else {
            dp_reserved
        }
    }
}

// Points pumps distinct num and count from nums.
struct Points {
    nums: IntoIter<i32>,
    current_num: Option<i32>,
}

impl Points {
    fn next(&mut self) -> Option<(i32, i32)> {
        match self.current_num {
            None => None,
            Some(num) => {
                let mut count = 1;
                for next_num in self.nums.by_ref() {
                    if num == next_num {
                        count += 1;
                    } else {
                        self.current_num = Some(next_num);
                        return Some((num, count));
                    }
                }
                self.current_num = None;
                Some((num, count))
            }
        }
    }
}
// @leet end
