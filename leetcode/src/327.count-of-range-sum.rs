pub struct Solution;

use std::collections::HashMap;
// 给定数组 nums, 求数组中区间和在 [lower, upper] 范围内的区间的个数.
//
impl Solution {
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
    //
    // 改进: 边构造 prefix_sums, 边计算, 以 i 为起点时, prefix_sums 只需要加入 i+1 的 prefix_sums
    // 这样可以不用检查下标,不过要维护有序需要进行插入排序, 直接用 Binary Index Tree 维护
    // prefix_sums 即可, 见下
    pub fn count_range_sum2(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
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

    // 还是先计算 prefix_sums[i] = sum(nums[0]...nums[i])
    // 对于 i 作为区间终点的区间,我们只需要考虑 j < i 的 prefix_sums[j]
    // lower <= prefix_sums[i] - prefix_sums[j] <= upper
    // prefix_sums[i] - upper <= prefix_sums[j] <= prefix_sums[i] - lower
    // 构造 Binary Indexed Tree 从前往后加,
    // 注意到原始的 prefix_sums 值可能非常大,需要的 BIT
    // 空间也会非常大,所以需要将可能的值做重新映射,
    // 我们查询 BIT 涉及的值只可能是 prefix_sums[i] - upper, prefix_sums[i] - lower
    // 而更新的值只涉及到 prefix_sums[i]
    // 即所有可能的值为 prefix_sums[i] - upper, prefix_sums[i], prefix_sums[i] - lower, 0 <= i < n,
    // 最多 3 * n 个, 我们把这些值映射到 [0, 3*n -1] 的空间中,
    // 用在映射空间中的值的下标代表这些值对 BIT 进行操作
    // 因些需要对值进行排序
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let (lower, upper) = (lower as i64, upper as i64);
        let mut prefix_sums = Vec::with_capacity(nums.len() + 1);
        let mut prev = 0;
        prefix_sums.push(prev);
        for num in nums.iter() {
            prev += *num as i64;
            prefix_sums.push(prev);
        }
        let mut bit_bounds = Vec::with_capacity(nums.len() * 3);
        for sum in prefix_sums.iter() {
            bit_bounds.push(*sum);
            bit_bounds.push(*sum - upper);
            bit_bounds.push(*sum - lower);
        }
        bit_bounds.sort();
        let bound_index = HashMap::<i64, usize>::from_iter(
            bit_bounds.iter().enumerate().map(|(i, v)| (*v, i + 1)),
        );

        let mut bit = BinaryIndexedTree::new(prefix_sums.len() * 3);
        let mut count = 0;
        for sum in prefix_sums.iter() {
            let index = bound_index.get(sum).unwrap();
            let lower_index = bound_index.get(&(*sum - upper)).unwrap();
            let upper_index = bound_index.get(&(*sum - lower)).unwrap();
            count += bit.get_sum(*upper_index) - bit.get_sum(*lower_index - 1);
            bit.update(*index, 1);
        }

        count
    }
}

struct BinaryIndexedTree {
    array: Vec<i32>,
}

impl BinaryIndexedTree {
    fn new(length: usize) -> Self {
        BinaryIndexedTree {
            array: vec![0; length + 1],
        }
    }
    fn update(&mut self, index: usize, delta: i32) {
        let mut x = index as i32;
        let len = self.array.len() as i32;
        while x < len {
            self.array[x as usize] += delta;
            x += x & -x;
        }
    }

    fn get_sum(&self, index: usize) -> i32 {
        let mut sum = 0;
        let mut x = index as i32;
        while x > 0 {
            sum += self.array[x as usize];
            x -= x & -x;
        }
        sum
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
