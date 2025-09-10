use std::collections::HashMap;

pub struct Solution;

// answers 是否森林里的一只兔子与它颜色相同的其它兔子有多少个，
// 问给定 answers，至少有多少只兔子。
//
// 对于给定一个颜色子的兔子群，它他说的 answer 都是相同的，假设
// answer = r, 则分配一个颜色给这个群，至多可以容纳 r+1 只说 answer = r 的兔子
// 因此有 c 只兔子说 r，当 c <= r + 1 时，需要分配一个颜色，
// c > r + 1 时，需要划分成 Ceil(c / (r + 1)) 个群，一个群有 r + 1 只
// 即 (c + r) / (r + 1) * (r + 1)
impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let mut answer_counts = HashMap::<i32, i32>::new();
        answers.iter().for_each(|answer| {
            answer_counts
                .entry(*answer)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        });

        let mut rabbit_count = 0;

        answer_counts.iter().for_each(|(r, c)| {
            // 有 c 只兔子说，与他们同颜色的兔子有 r 个
            // 说 r 个同颜色，则可以至多容纳 r + 1 个兔子
            rabbit_count += (c + r) / (r + 1) * (r + 1)
        });

        rabbit_count
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn test_examples() {
        assert_eq!(Solution::num_rabbits(vec![1, 1, 2]), 5);
        assert_eq!(Solution::num_rabbits(vec![10, 10, 10]), 11);
    }
}
