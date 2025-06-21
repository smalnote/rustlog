use std::collections::BTreeSet;

// 实现 SummaryRanges 持续地添加非负整数,调用 get_intervals 时用不相交的区间表示
// 目前已经加的数, 如:
// 1, 3 => [1, 1], [3, 3]
// 2 => [1, 3]
// 5 => [1, 3], [5]
// 0 => [0, 3]
// 采用 TreeSet 维护以区间开头排序的集合
// 对于 插入 x, 需要找到下界和上界
// 下界为最后一个 <= x, .ordered_set.range((Unbounded, Included((value, 0)))).next_back();
// 上界为第一个 >= x, .ordered_set.range(Included(value, 0), Unbounded).next();
#[derive(Default, Debug)]
pub struct SummaryRanges {
    ordered_set: BTreeSet<(usize, usize)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
use std::ops::Bound::{Included, Unbounded};
impl SummaryRanges {
    pub fn new() -> Self {
        Self {
            ordered_set: BTreeSet::new(),
        }
    }

    // 考虑情况:
    // x => ] x [
    pub fn add_num(&mut self, value: i32) {
        let value = value as usize;
        let left = self
            .ordered_set
            .range((Unbounded, Included((value, 0))))
            .next_back();
        let right = self
            .ordered_set
            .range((Included((value, 0)), Unbounded))
            .next();
        match (left, right) {
            (None, None) => {
                self.ordered_set.insert((value, value));
            }
            (Some(&left), None) => {
                // left[0, 1] , left.0 <= value
                // left.1 < value
                if left.1 + 1 == value {
                    self.ordered_set.remove(&left);
                    self.ordered_set.insert((left.0, value));
                } else if left.1 < value {
                    self.ordered_set.insert((value, value));
                }
            }
            (None, Some(&right)) => {
                // value, right[0, 1] right.0 >= value
                if value + 1 == right.0 {
                    self.ordered_set.remove(&right);
                    self.ordered_set.insert((value, right.1));
                } else if value < right.0 {
                    self.ordered_set.insert((value, value));
                }
            }
            (Some(&left), Some(&right)) => {
                // left.0 <= left.1 <= value
                // value <= right.0 <= right.1
                // left.1 < right.0
                if left.1 + 1 == value && value + 1 == right.0 {
                    self.ordered_set.remove(&left);
                    self.ordered_set.remove(&right);
                    self.ordered_set.insert((left.0, right.1));
                } else if left.1 + 1 == value {
                    self.ordered_set.remove(&left);
                    self.ordered_set.insert((left.0, value));
                } else if value + 1 == right.0 {
                    self.ordered_set.remove(&right);
                    self.ordered_set.insert((value, right.1));
                } else if left.1 + 1 < value && value + 1 < right.0 {
                    self.ordered_set.insert((value, value));
                }
            }
        };
    }

    pub fn get_intervals(&self) -> Vec<Vec<i32>> {
        Vec::from_iter(
            self.ordered_set
                .iter()
                .map(|&(lo, hi)| vec![lo as i32, hi as i32]),
        )
    }
}

#[cfg(test)]
mod test {
    use super::SummaryRanges;
    #[test]
    fn test_example_3() {
        let mut sr = SummaryRanges::new();
        sr.add_num(6);
        assert_eq!(sr.get_intervals(), vec![vec![6, 6]]);
        sr.add_num(6);
        assert_eq!(sr.get_intervals(), vec![vec![6, 6]]);
        sr.add_num(0);
        assert_eq!(sr.get_intervals(), vec![vec![0, 0], vec![6, 6]]);
        sr.add_num(4);
        assert_eq!(sr.get_intervals(), vec![vec![0, 0], vec![4, 4], vec![6, 6]]);
        sr.add_num(8);
        assert_eq!(
            sr.get_intervals(),
            vec![vec![0, 0], vec![4, 4], vec![6, 6], vec![8, 8]]
        );
        sr.add_num(7);
        assert_eq!(sr.get_intervals(), vec![vec![0, 0], vec![4, 4], vec![6, 8]]);
        sr.add_num(6);
        assert_eq!(sr.get_intervals(), vec![vec![0, 0], vec![4, 4], vec![6, 8]]);
        sr.add_num(4);
        assert_eq!(sr.get_intervals(), vec![vec![0, 0], vec![4, 4], vec![6, 8]]);
        sr.add_num(7);
        assert_eq!(sr.get_intervals(), vec![vec![0, 0], vec![4, 4], vec![6, 8]]);
        sr.add_num(5);
        assert_eq!(sr.get_intervals(), vec![vec![0, 0], vec![4, 8]]);
    }
    #[test]
    fn test_example_2() {
        let mut sr = SummaryRanges::new();
        sr.add_num(1);
        assert_eq!(sr.get_intervals(), vec![vec![1, 1]]);
        sr.add_num(0);
        assert_eq!(sr.get_intervals(), vec![vec![0, 1]]);
    }
    #[test]
    fn test_example() {
        let mut sr = SummaryRanges::new();
        sr.add_num(1);
        assert_eq!(sr.get_intervals(), vec![vec![1, 1]]);
        sr.add_num(3);
        assert_eq!(sr.get_intervals(), vec![vec![1, 1], vec![3, 3]]);
        sr.add_num(7);
        assert_eq!(sr.get_intervals(), vec![vec![1, 1], vec![3, 3], vec![7, 7]]);
        sr.add_num(2);
        assert_eq!(sr.get_intervals(), vec![vec![1, 3], vec![7, 7]]);
        sr.add_num(6);
        assert_eq!(sr.get_intervals(), vec![vec![1, 3], vec![6, 7]]);
    }
}
