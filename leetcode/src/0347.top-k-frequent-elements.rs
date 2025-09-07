pub struct Solution;

use std::collections::HashMap;
// 返回数组中出现频率最高的 k 个数
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut num_count = HashMap::new();
        for num in nums {
            num_count
                .entry(num)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        let mut num_count: Vec<(i32, i32)> = num_count.into_iter().collect();
        for i in (0..=(if num_count.len() > 1 {
            num_count.len() - 2
        } else {
            0
        }) / 2)
            .rev()
        {
            // swap down
            Self::swap_down(&mut num_count, i);
        }

        let mut result = Vec::with_capacity(k as usize);
        for _ in 0..k {
            let last = num_count.len() - 1;
            num_count.swap(0, last);
            result.push(num_count.pop().unwrap().0);
            Self::swap_down(&mut num_count, 0);
        }
        result
    }

    fn swap_down(num_count: &mut [(i32, i32)], mut curr: usize) {
        while curr * 2 + 1 < num_count.len() {
            let mut max = curr * 2 + 1;
            if max + 1 < num_count.len() && num_count[max].1 < num_count[max + 1].1 {
                max += 1;
            }
            if num_count[curr].1 < num_count[max].1 {
                num_count.swap(curr, max);
                curr = max;
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn test_example() {
        assert_eq!(
            Solution::top_k_frequent(vec![1, 1, 1, 2, 3, 2], 2),
            vec![1, 2]
        );
    }
}
