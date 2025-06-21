pub struct Solution;

// 给定一组开头都不同区间,一个区间 i 的 right interval j 有 start_j >= end_i
// start_j 在所有可能中最小, j 可能等于 i
// 将区间按 start 升序排,带上原始坐标, 对每个区间的 end 在排序好的区间中做二分查找
impl Solution {
    pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let mut sorted = Vec::from_iter(
            intervals
                .iter()
                .enumerate()
                .map(|(i, interval)| (interval[0], interval[1], i)),
        );
        sorted.sort();
        let mut right_intervals = vec![-1; intervals.len()];
        for (i, v) in intervals.iter().enumerate() {
            match sorted.binary_search_by(|(start, _, _)| start.cmp(&v[1])) {
                Ok(j) => right_intervals[i] = sorted[j].2 as i32,
                Err(j) => {
                    if j < sorted.len() {
                        right_intervals[i] = sorted[j].2 as i32;
                    }
                }
            }
        }
        right_intervals
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn test_example() {
        assert_eq!(
            Solution::find_right_interval(vec![
                vec![-100, -98],
                vec![-99, -97],
                vec![-98, -96],
                vec![-97, -95]
            ]),
            vec![2, 3, -1, -1]
        );
        assert_eq!(Solution::find_right_interval(vec![vec![1, 2]]), vec![-1]);
    }
}
