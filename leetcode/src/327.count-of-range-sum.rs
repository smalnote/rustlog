pub struct Solution;

// 给定数组 nums, 求数组中区间和在 [lower, upper] 范围内的区间的个数.
//
// 先求前缀各 prefix_sums[i] 表示区间 [0, i) 的和,
// 则求任意区间和 sums[i][j] = prefix_sums[j+1] - prefix_sums[i]
// 选定一个 i, 查找以 i 为端点的区间和在 [lower, upper] 的区间
// 以 i 为区间起点, 则找 j > i 且有 lower <= prefix_sums[j] - prefix_sums[i] <= upper
// 则 lower + prefix_sums[i] <= prefix_sums[j] <= upper + prefix_sums[i]
// 以 i 为区间终点,则找 j < i 且有 lower <= prefix_sums[i] - prefix_sums[j] <= upper
// 则 prefix_sums[i] - upper <= prefix_sums[j] <= prefix_sums[i] - lower
// 注意 i == j 时, [i, j) 表示空区间,不能计数
// 注意到 i, j 是对称的, i 作为起点找 j 算了 [i, j] 一次, j 作为终点找 i 计算了 [i, j] 一次
// 实际只需要把 i 作为起点计算即可, i 作为终点包含在其 i  作为起点的情况
// 把 prefix_sums[i] 用元组 (sum, i) 表示, 再按 sum 排列,在 prefix_sums 中进行二分查找
impl Solution {
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let (lower, upper) = (lower as i64, upper as i64);
        let mut predix_sums = vec![(0, 0); nums.len() + 1];
        for (i, num) in nums.iter().enumerate() {
            predix_sums[i + 1] = (predix_sums[i].0 + *num as i64, i + 1);
        }
        predix_sums.sort_by(|(a, _), (b, _)| a.cmp(b));

        let mut count = 0;

        for (sum, i) in predix_sums.iter() {
            // i as start
            let right = {
                // 在 prefix_sums 查找 prefix_sums[i].0 <= sum + upper 且 i 最大的元素
                // 如果都大于,则不存在,因此改为查找 > sum + upper 且 i 最小的元素
                let target = sum + upper;
                let (mut lo, mut hi) = (0, predix_sums.len());
                while lo < hi {
                    let mid = (lo + hi) / 2;
                    if predix_sums[mid].0 <= target {
                        lo = mid + 1;
                    } else {
                        hi = mid;
                    }
                }
                // lo 是第一个大于目标的元素的下标,则取右开区间 prefix_sums[0..lo] 都是小于等于目标的
                lo
            };
            let left = {
                // 在 prefix_sums 查找 prefix_sums[i].0 >= sum + lower 且 i 最小的元素
                // 如果都小于,则不存在,则返回 prefix_sums.len(), prefix_sums[len()..] 取到空
                let target = sum + lower;
                let (mut lo, mut hi) = (0, predix_sums.len());
                while lo < hi {
                    let mid = (lo + hi) / 2;
                    if predix_sums[mid].0 >= target {
                        hi = mid;
                    } else {
                        lo = mid + 1;
                    }
                }
                lo
            };
            for j in &predix_sums[left..right] {
                if j.1 > *i {
                    count += 1;
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn test_example() {
        assert_eq!(
            Solution::count_range_sum(vec![-2147483647, 0, -2147483647, 2147483647], -564, 3864),
            3
        );
        assert_eq!(
            Solution::count_range_sum(vec![-3, 1, 2, -2, 2, -1], -3, -1),
            7
        );
        assert_eq!(Solution::count_range_sum(vec![-2, 5, -1], -2, 2), 3);
        assert_eq!(Solution::count_range_sum(vec![0, 0], 0, 0), 3);
        assert_eq!(Solution::count_range_sum(vec![0], 0, 0), 1);
    }
}
