pub struct Solution;

/// rungs 代表一个高度严格递增的阶梯序列， dist 表示每次可以跨越的阶梯高度，
/// 从高度 0 开始，登上最后一个阶梯，如果两个阶梯之间跨试过大，可以插入一个新的，
/// 问要登上最后一个阶梯，最少需要插入多少个新的阶梯。
impl Solution {
    pub fn add_rungs(rungs: Vec<i32>, dist: i32) -> i32 {
        let mut current = 0;
        let mut add = 0;
        for rung in rungs.iter() {
            if current + dist < *rung {
                add += (*rung - current - 1) / dist;
            }
            current = *rung
        }
        add
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn test_examples() {
        assert_eq!(Solution::add_rungs(vec![1, 3, 4, 10], 2), 2);
        assert_eq!(Solution::add_rungs(vec![3, 6, 8, 10], 3), 0);
        assert_eq!(Solution::add_rungs(vec![3, 4, 6, 7], 2), 1);
        assert_eq!(Solution::add_rungs(vec![10000], 1), 9999);
    }
}
